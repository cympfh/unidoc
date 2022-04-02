pub mod entity;
pub mod parser;
pub mod translator;

use std::io::{self, Read};
use structopt::StructOpt;

fn read() -> String {
    let mut content = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut content).unwrap();
    if !content.ends_with('\n') {
        content += "\n"
    }
    content
}

fn write(buf: &String) {
    println!("{}", buf);
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "debug")]
    pub debug: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.debug {
        println!(">>> opt = {:?}", &opt);
    }
    let content = read();
    if let Ok(markdown) = parser::markdown(content.as_str()) {
        if opt.debug {
            println!(">>> markdown = {:?}", &markdown);
        }
        let tr = translator::Translator::new(true);
        let html = tr.markdown(&markdown);
        write(&html);
    } else {
        eprintln!("Something critical error");
    }
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
