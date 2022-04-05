pub mod entity;
pub mod io;
pub mod parser;
pub mod template;
pub mod translator;

use crate::translator::Translator;
use std::error::Error;
use std::path::Path;
use structopt::StructOpt;

use crate::template::simple;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "debug")]
    pub debug: bool,
    #[structopt(long = "out", short = "o")]
    pub output: Option<String>,
    #[structopt(long = "standalone", short = "s")]
    pub standalone: bool,
    #[structopt(long = "compact", short = "c")]
    pub compact: bool,
    #[structopt(long = "indent", default_value = "2")]
    pub indent: usize,
    #[structopt(name = "input")]
    pub input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    if opt.debug {
        eprintln!(">>> opt = {:?}", &opt);
    }

    let filedir: Option<String> = opt
        .input
        .as_ref()
        .map(|input| {
            Path::new(&input)
                .parent()
                .map(|path| String::from(path.to_str().unwrap()))
        })
        .flatten();

    let content = io::read(&opt.input)?;
    if let Ok(markdown) = parser::markdown(content.as_str()) {
        if opt.debug {
            eprintln!(">>> markdown = {:?}", &markdown);
        }
        let tr = Translator::new(filedir);
        let (title, htmldoc) = tr.markdown(&markdown);
        if opt.debug {
            eprintln!(">>> htmldoc = {:?}", &htmldoc);
        }
        let body = htmldoc.show(opt.compact, opt.indent);
        let html = if opt.standalone {
            simple(title, body)?
        } else {
            body
        };
        io::write(&opt.output, &html)?;
    } else {
        eprintln!("Something critical error");
    }
    Ok(())
}

#[cfg(test)]
mod test_main {

    use crate::parser;
    use crate::translator::Translator;

    macro_rules! assert_convert {
        ($compact:expr, $markdown:expr, $title:expr, $body:expr) => {
            let mkd = parser::markdown($markdown).unwrap();
            let tr = Translator::new(None);
            let (title, htmldoc) = tr.markdown(&mkd);
            let body = htmldoc.show($compact, 2);
            assert_eq!((title, body), (String::from($title), String::from($body)));
        };
        (compact; $markdown:expr, $title:expr, $body:expr) => {
            assert_convert!(true, $markdown, $title, $body)
        };
    }

    #[test]
    fn test_convert() {
        assert_convert!(compact; "# h1\n", "h1", "<h1>h1</h1>\n");
        assert_convert!(compact; "## h2\n", "h2", "<h2>h2</h2>\n");
        assert_convert!(compact; "a  b\nc\n", "a b c", "<p>a b c</p>\n");
        assert_convert!(compact; "a  \nb\nc\n\n---\n", "a  b c", "<p>a <br /> b c</p><hr />\n");
        assert_convert!(compact; "*a* <!-- b -->\n",
            "a ",
            "<p><em>a</em> <!-- b --></p>\n");
        assert_convert!(compact; "- a\n- b\n- c\n",
            "",
            "<ul><li>a</li><li>b</li><li>c</li></ul>\n"
        );
        assert_convert!(compact; "| A |\n|:-:|\n| a |\n",
            "",
            "<table><thead><tr class=header><th>A</th></tr></thead><tbody><tr class=odd><td>a</td></tr></tbody></table>\n"
        );
    }
}
