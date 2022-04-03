pub mod entity;
pub mod parser;
pub mod template;
pub mod translator;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read, Write};
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
    #[structopt(name = "input")]
    pub input: Option<String>,
}

fn read(opt: &Opt) -> io::Result<String> {
    let mut content = String::new();
    if let Some(input) = &opt.input {
        let file = File::open(&input).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content)?;
    }
    if !content.ends_with('\n') {
        content += "\n"
    }
    Ok(content)
}

fn write(opt: &Opt, buf: &String) -> io::Result<()> {
    if let Some(output) = &opt.output {
        let mut file = File::create(&output)?;
        write!(file, "{}", buf)?;
    } else {
        println!("{}", buf);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    if opt.debug {
        println!(">>> opt = {:?}", &opt);
    }
    let content = read(&opt)?;
    if let Ok(markdown) = parser::markdown(content.as_str()) {
        if opt.debug {
            println!(">>> markdown = {:?}", &markdown);
        }
        let tr = translator::Translator::new(true);
        let (title, body) = tr.markdown(&markdown);
        let html = if opt.standalone {
            simple(title, body)?
        } else {
            body
        };
        write(&opt, &html)?;
    } else {
        eprintln!("Something critical error");
    }
    Ok(())
}

#[cfg(test)]
mod test_main {

    use crate::parser;
    use crate::translator;

    macro_rules! assert_convert {
        ($compact:expr, $markdown:expr, $html:expr) => {
            let tr = translator::Translator::new($compact);
            assert_eq!(
                tr.markdown(&parser::markdown($markdown).unwrap()),
                String::from($html)
            );
        };
        (compact; $markdown:expr, $html:expr) => {
            assert_convert!(true, $markdown, $html)
        };
        (full; $markdown:expr, $html:expr) => {
            assert_convert!(false, $markdown, $html)
        };
    }

    #[test]
    fn test_convert() {
        assert_convert!(compact; "# h1\n", "<h1>h1</h1>\n");
        assert_convert!(compact; "## h2\n", "<h2>h2</h2>\n");
        assert_convert!(compact; "a  b\nc\n", "<p>a b c</p>\n");
        assert_convert!(compact; "a  \nb\nc\n\n---\n", "<p>a <br /> b c</p><hr />\n");
        assert_convert!(compact; "*a* <!-- b -->\n", "<p><em>a</em> <!-- b --></p>\n");
        assert_convert!(
            compact;
            "- a\n- b\n- c\n",
            "<ul><li>a</li><li>b</li><li>c</li></ul>\n"
        );
        assert_convert!(compact;
            "| A |\n|:-:|\n| a |\n",
            "<table><thead><tr class=header><th>A</th></tr></thead><tbody><tr class=odd><td>A</td></tr></tbody></table>\n"
        );
    }

    // #[test]
    // fn test_examples_full() {
    //     use std::fs::read_to_string;
    //     let content = read_to_string("./examples/full.md").unwrap();
    //     let expected = read_to_string("./examples/full.html").unwrap();
    //     assert_convert!(content.as_str(), expected.as_str());
    // }
}
