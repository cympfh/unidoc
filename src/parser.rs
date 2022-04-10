use crate::entity::markdown::{
    Align, Block, Inline, List, ListItem, ListOrderType, Markdown, Text,
};
use nom::branch::alt;
use nom::bytes::complete::{
    is_not, tag, take, take_until, take_while, take_while1, take_while_m_n,
};
use nom::character::complete::{
    alpha1, alphanumeric0, digit1, line_ending, multispace0, space0, space1,
};
use nom::combinator::{map, map_parser, not, opt, peek, success};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

type ParseResult<'a, T> = IResult<&'a str, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn markdown(input: &str) -> Result<Markdown, ParseError> {
    if let Ok((rest, doc)) = parse_markdown(input) {
        if !rest.is_empty() {
            eprintln!("Parsed: {:?}", &doc);
            eprintln!("Left: {:?}", &rest);
            Err(ParseError(String::from(rest)))
        } else {
            Ok(doc)
        }
    } else {
        Err(ParseError(String::from(
            "Something Critical Error in Parsing Markdown",
        )))
    }
}

fn parse_markdown(input: &str) -> ParseResult<Markdown> {
    let parse_blocks = many0(preceded(multispace0, parse_block));
    let mut parse_all = terminated(parse_blocks, multispace0);
    parse_all(input)
}

fn parse_block(input: &str) -> ParseResult<Block> {
    let parse_hr = map(preceded(tag("---"), line_ending), |_| Block::HorizontalRule);

    let parse_heading = map(
        tuple((take_while_m_n(1, 6, |c| c == '#'), space1, parse_text_line)),
        |(hashes, _, text)| Block::Heading(hashes.len(), text),
    );

    let parse_code = map(
        pair(
            delimited(tag("```"), alphanumeric0, line_ending),
            terminated(terminated(take_until("```"), tag("```")), line_ending),
        ),
        |(lang, code): (&str, &str)| {
            if lang.is_empty() {
                Block::Code(None, code.to_string())
            } else {
                Block::Code(Some(lang.to_string()), code.to_string())
            }
        },
    );

    let parse_listblock = map(parse_list(0), |list| Block::ListBlock(list));

    let parse_paragraph = map(
        terminated(many1(parse_text_line), alt((line_ending, success("")))), // TODO(success 要らない?)
        |texts: Vec<Vec<Inline>>| Block::Paragraph(texts.into_iter().flatten().collect()),
    );

    let quoting = pair(tag(">"), space0);
    let parse_quoted = map(
        terminated(
            many1(preceded(quoting, parse_text_line)),
            alt((line_ending, success(""))),
        ),
        |texts: Vec<Vec<Inline>>| Block::Quoted(texts.into_iter().flatten().collect()),
    );

    let parse_import = map(
        terminated(delimited(tag("@("), is_not(")"), tag(")")), line_ending),
        |path: &str| Block::Import(path.to_string()),
    );

    let parse_hyperlink = map(
        terminated(
            delimited(
                pair(tag("{{"), space0),
                is_not(" }"),
                pair(space0, tag("}}")),
            ),
            line_ending,
        ),
        |url: &str| Block::HyperLink(url.to_string()),
    );

    let parse_code_import = map(
        terminated(
            pair(
                delimited(tag("@["), opt(is_not("]")), tag("]")),
                delimited(tag("("), is_not(")"), tag(")")),
            ),
            line_ending,
        ),
        |(lang, path): (Option<&str>, &str)| {
            Block::CodeImport(lang.map(|s| s.to_string()), path.to_string())
        },
    );

    let parse_mathjax = map(
        terminated(delimited(tag("$$"), parse_tex, tag("$$")), line_ending),
        |tex| Block::MathJax(tex.to_string()),
    );

    alt((
        parse_hr,
        parse_heading,
        parse_code,
        parse_listblock,
        parse_table,
        parse_quoted,
        parse_import,
        parse_code_import,
        parse_hyperlink,
        parse_mathjax,
        parse_paragraph,
    ))(input)
}

fn parse_table(input: &str) -> ParseResult<Block> {
    /// | VALUE | VALUE | ... | VALUE | NEWLINE
    fn parse_row(input: &str) -> ParseResult<Vec<Text>> {
        let parse_a_value = terminated(delimited(space0, parse_text, space0), tag("|"));
        delimited(tag("|"), many1(parse_a_value), line_ending)(input)
    }

    /// | --- | :---- |:---:| ---:| NEWLINE
    fn parse_rules(input: &str) -> ParseResult<Vec<Align>> {
        let is_hyphen = |c: char| c == '-';
        // RULE ::= "---" | ":---:" | ":---" | "---:"
        let parse_rule = alt((
            map(delimited(tag(":"), take_while(is_hyphen), tag(":")), |_| {
                Align::Center
            }),
            map(pair(tag(":"), take_while(is_hyphen)), |_| Align::Left),
            map(pair(take_while(is_hyphen), tag(":")), |_| Align::Right),
            map(take_while1(is_hyphen), |_| Align::Left),
        ));
        // ::= SPACE0 RULE SPACE0 "|"
        let parse_a_rule = map(
            tuple((space0, parse_rule, space0, tag("|"))),
            |(_, align, _, _)| align,
        );
        delimited(tag("|"), many1(parse_a_rule), line_ending)(input)
    }

    let parse_headers = pair(parse_row, parse_rules);

    let parse_table_with_header = map(
        pair(parse_headers, many1(parse_row)),
        |((headers, aligns), body)| {
            let mut content = vec![headers];
            content.append(&mut body.clone());
            Block::Table(aligns, content, true)
        },
    );
    let parse_table_without_header = map(many1(parse_row), |body| {
        let m = body[0].len();
        let aligns = (0..m).map(|_| Align::Left).into_iter().collect();
        Block::Table(aligns, body, false)
    });

    alt((parse_table_with_header, parse_table_without_header))(input)
}

/// Parse text without newline
fn parse_text(input: &str) -> ParseResult<Text> {
    let parse_emphasis_and_strong = map(
        map_parser(delimited(tag("***"), is_not("***"), tag("***")), parse_text),
        Inline::EmphasisAndStrong,
    );
    let parse_strong = map(
        map_parser(
            alt((
                delimited(tag("**"), is_not("**"), tag("**")),
                delimited(tag("__"), is_not("__"), tag("__")),
            )),
            parse_text,
        ),
        Inline::Strong,
    );
    let parse_emphasis = map(
        map_parser(
            alt((
                delimited(tag("*"), is_not("*"), tag("*")),
                delimited(tag("_"), is_not("_"), tag("_")),
            )),
            parse_text,
        ),
        Inline::Emphasis,
    );
    let parse_deleted = map(
        map_parser(delimited(tag("~~"), is_not("~~"), tag("~~")), parse_text),
        Inline::Deleted,
    );
    let parse_code = map(delimited(tag("`"), is_not("`"), tag("`")), |text: &str| {
        Inline::Code(text.to_string())
    });
    let parse_image = map(
        pair(
            delimited(tag("!["), opt(is_not("]")), tag("]")),
            delimited(tag("("), is_not(")"), tag(")")),
        ),
        |(alt, link): (Option<&str>, &str)| {
            Inline::Image(alt.unwrap_or(&"").to_string(), link.to_string())
        },
    );
    let parse_link = map(
        pair(
            delimited(tag("["), parse_text, tag("]")),
            delimited(tag("("), is_not(")"), tag(")")),
        ),
        |(text, url): (Text, &str)| Inline::Link(text, url.to_string()),
    );
    let parse_hyperlink = map(
        delimited(tag("[["), is_not("]]"), tag("]]")),
        |url: &str| Inline::HyperLink(url.to_string()),
    );
    let parse_comment = map(
        delimited(tag("<!--"), is_not("-->"), tag("-->")),
        |text: &str| Inline::Comment(text.to_string()),
    );
    let parse_mathjax = map(delimited(tag("$"), parse_tex, tag("$")), |tex| {
        Inline::MathJax(tex)
    });

    many1(preceded(
        space0,
        alt((
            parse_hyperlink,
            parse_link,
            parse_image,
            parse_emphasis_and_strong,
            parse_strong,
            parse_emphasis,
            parse_deleted,
            parse_code,
            parse_comment,
            parse_mathjax,
            parse_plaintext,
        )),
    ))(input)
}

/// Parse one-line text ending with a newline
fn parse_text_line(input: &str) -> ParseResult<Text> {
    let (input, (text, newline)) = pair(parse_text, opt(tag("  ")))(input)?;
    let (input, _) = line_ending(input)?;
    let mut text = text;
    if let Some(_) = newline {
        text.push(Inline::Newline);
    }
    Ok((input, text))
}

fn parse_plaintext(input: &str) -> ParseResult<Inline> {
    let safe_one_char = preceded(
        not(alt((
            tag(" "),
            tag("*"),
            tag("`"),
            tag("~~"),
            tag("["),
            tag("]"),
            tag("!["),
            tag("|"),
            tag("\\"),
            tag("\n"),
            tag("\r"),
            tag("<!--"),
        ))),
        take(1u8),
    );
    let escaped_char = map(
        alt((
            tag("\\`"),
            tag("\\~"),
            tag("\\<"),
            tag("\\>"),
            tag("\\|"),
            tag("\\ "),
            tag("\\!"),
            tag("\\["),
            tag("\\]"),
            tag("\\*"),
            tag("\\\\"),
        )),
        |e: &str| &e[1..2],
    );
    map(many1(alt((safe_one_char, escaped_char))), |v| {
        Inline::Plaintext(v.join(""))
    })(input)
}

fn parse_list<'r>(indent: usize) -> impl FnMut(&'r str) -> ParseResult<'r, List> {
    fn parse_bullet(input: &str) -> ParseResult<ListOrderType> {
        terminated(
            alt((
                map(alt((tag("-"), tag("*"), tag("+"))), |_| {
                    ListOrderType::Unordered
                }),
                map(pair(digit1, tag(".")), |_| ListOrderType::OrderedNumbers),
                map(pair(alpha1, tag(".")), |_| ListOrderType::OrderedAlphabets),
            )),
            space1,
        )(input)
    }

    // one list-item ::= INDENT, BULLET, CHECKBOX, TEXT, CHILDREN
    fn parse_listitem<'r>(
        indent: usize,
    ) -> impl FnMut(&'r str) -> ParseResult<'r, (ListOrderType, ListItem)> {
        let is_space = |c: char| c == ' ' || c == '\t';
        let parse_indent = take_while_m_n(indent, indent, is_space);
        let checkbox = alt((
            map(alt((tag("[ ]"), tag("[]"))), |_| false),
            map(alt((tag("[x]"), tag("[X]"))), |_| true),
        ));
        map(
            tuple((
                parse_indent,
                parse_bullet,
                opt(delimited(space0, checkbox, space0)),
                parse_text_line,
                opt(parse_list_children(indent)),
            )),
            |(_indent, listtype, checked, text, children): (
                &str,
                ListOrderType,
                Option<bool>,
                Text,
                Option<List>,
            )| { (listtype, ListItem::new(checked, text, children)) },
        )
    }

    // list-children ::= parse_list(MORE_INDENT)
    fn parse_list_children<'r>(indent: usize) -> impl FnMut(&'r str) -> ParseResult<'r, List> {
        let is_space = |c: char| c == ' ' || c == '\t';
        let mut peek_nextindent = peek(map(
            take_while_m_n::<_, &str, _>(indent + 1, indent + 1000, is_space),
            |nextindent| nextindent.len(),
        ));
        move |input: &str| {
            let (input, m) = peek_nextindent(input)?;
            parse_list(m)(input)
        }
    }

    map(many1(parse_listitem(indent)), |ps| {
        let listtype = ps[0].0;
        let items: Vec<_> = ps.iter().map(|(_, item)| item).cloned().collect();
        List::new(listtype, items)
    })
}

/// Inner of $...$, $$...$$
fn parse_tex(input: &str) -> ParseResult<String> {
    let safe_one_char = preceded(not(alt((tag("$"), tag("\\$")))), take(1u8));
    map(many1(alt((safe_one_char, tag("\\$")))), |v| v.join(""))(input)
}

#[cfg(test)]
mod test_parser {

    use crate::entity::markdown::*;
    use crate::parser::markdown;

    macro_rules! cannot_parse {
        ($markdown:expr) => {
            let code = markdown($markdown);
            assert!(code.is_err(), "code: {:?}", code);
        };
        ($markdown:expr, $description:expr) => {
            cannot_parse!($markdown)
        };
    }
    macro_rules! assert_parse {
        ($markdown:expr, $blocks:expr) => {
            assert_eq!(markdown($markdown), Ok($blocks), $markdown);
        };
    }
    macro_rules! text {
        ($str:expr) => {
            (Inline::Plaintext(String::from($str)))
        };
    }
    macro_rules! comment {
        ($str:expr) => {
            (Inline::Comment(String::from($str)))
        };
    }
    macro_rules! codeblock {
        ($code:expr) => {
            Block::Code(None, $code.to_string())
        };
        ($lang:expr, $code:expr) => {
            Block::Code(Some($lang.to_string()), $code.to_string())
        };
    }
    macro_rules! p {
        ( $( $text:expr ),* $( , )? ) => {
            Block::Paragraph(vec![ $( $text ),* ])
        }
    }
    macro_rules! q {
        ( $( $text:expr ),* $( , )? ) => {
            Block::Quoted(vec![ $( $text ),* ])
        }
    }
    macro_rules! listblock {
        ( $listtype:expr ; $( ( $checked:expr , $label:expr , $children:expr ) ),* $( , )? ) => {
            Block::ListBlock( list!( $listtype ; $( ($checked , $label , $children) ),* ) )
        };
    }
    macro_rules! list {
        ( $listtype:expr ; $( ( $checked:expr , $label:expr , $children:expr ) ),* $( , )? ) => {
            List::new(
                $listtype,
                vec![ $( ListItem::new($checked, $label, $children) ),* ]
            )
        };
    }

    #[test]
    fn test_empty() {
        assert_parse!("", vec![]);
        assert_parse!(" \t\n\r", vec![]);
    }

    #[test]
    fn test_headings() {
        assert_parse!(
            "# title\n\n## subtitle\n---\n",
            vec![
                Block::Heading(1, vec![text!("title")]),
                Block::Heading(2, vec![text!("subtitle")]),
                Block::HorizontalRule
            ]
        );
        assert_parse!(
            "### h3 title\n",
            vec![Block::Heading(3, vec![text!("h3"), text!("title")]),]
        );
        assert_parse!(
            "### **bold title** ![icon](icon.png)\n",
            vec![Block::Heading(
                3,
                vec![
                    Inline::Strong(vec![text!("bold"), text!("title"),]),
                    Inline::Image(String::from("icon"), String::from("icon.png")),
                ]
            )]
        );
    }

    #[test]
    fn test_code() {
        assert_parse!(
            "In-line code: `f(x) = x^2`.\n",
            vec![Block::Paragraph(vec![
                text!("In-line"),
                text!("code:"),
                Inline::Code(String::from("f(x) = x^2")),
                text!("."),
            ])]
        );
    }

    #[test]
    fn test_text() {
        assert_parse!(
            "a  \nb\n",
            vec![p! {
                text!("a"),
                Inline::Newline,
                text!("b"),
            }]
        );
        assert_parse!(
            "![](image)\n",
            vec![p! {
                Inline::Image(
                    String::new(),
                    String::from("image"),
                ),
            }]
        );
        assert_parse!(
            "![alt](image)\n",
            vec![p! {
                Inline::Image(
                    String::from("alt"),
                    String::from("image"),
                ),
            }]
        );
        assert_parse!(
            "[text](link)\n",
            vec![p! {
                Inline::Link(
                    vec![text!("text")],
                    String::from("link"),
                ),
            }]
        );
        assert_parse!(
            "this is a [[hyperlink]]\n",
            vec![p! {
                text!("this"),
                text!("is"),
                text!("a"),
                Inline::HyperLink(String::from("hyperlink")),
            }]
        );
        assert_parse!(
            "Hello *world* **!** \\*\\!\n\n",
            vec![p! {
                text!("Hello"),
                Inline::Emphasis(vec![text!("world")]),
                Inline::Strong(vec![text!("!")]),
                text!("*!"),
            }]
        );
        assert_parse!(
            "__Hello__\n",
            vec![p! {
                Inline::Strong(vec![text!("Hello")]),
            }]
        );
        assert_parse!(
            "_Hello_ __Wor ld__ ***!***\n",
            vec![p! {
                Inline::Emphasis(vec![text!("Hello")]),
                Inline::Strong(vec![text!("Wor"), text!("ld")]),
                Inline::EmphasisAndStrong(vec![text!("!")]),
            }]
        );
        assert_parse!(
            "~~Hello~~\n",
            vec![p! { Inline::Deleted(vec![text!("Hello")]) }]
        );
        assert_parse!("~Hello~\n", vec![p! { text!("~Hello~") }]);
        assert_parse!("~Hello\n", vec![p! { text!("~Hello") }]);
    }

    #[test]
    fn test_escape() {
        assert_parse!(
            "a\\*b\n",
            vec![p! {
                text!("a*b"),
            }]
        );
        assert_parse!(
            "\\[\\]\\<\\>\\~\\*\\!\\|\\\\\n",
            vec![p! {
                text!("[]<>~*!|\\"),
            }]
        );
    }

    #[test]
    fn test_paragraph() {
        cannot_parse!("Hi", "Markdown must ends with newline");
        assert_parse!(
            "Hi\n",
            vec![p! {
                text!("Hi"),
            }]
        );
        assert_parse!(
            "Hi\n\n",
            vec![p! {
                text!("Hi"),
            }]
        );
        assert_parse!(
            "This\nis\na paragraph\n\n",
            vec![p! {
                text!("This"),
                text!("is"),
                text!("a"),
                text!("paragraph"),
            }]
        );
        assert_parse!(
            "Paragraph1\n\nParagraph2  \npara2\n\n\npara3\n\n\n",
            vec![
                p! { text!("Paragraph1") },
                p! {
                    text!("Paragraph2"),
                    Inline::Newline,
                    text!("para2"),
                },
                p! { text!("para3") },
            ]
        );
    }

    #[test]
    fn test_quote() {
        assert_parse!(
            "> quote quote.\n",
            vec![q! {
                text!("quote"),
                text!("quote."),
            }]
        );
        assert_parse!(
            "> quote quote.\n> *second* line.\n\n",
            vec![q! {
                text!("quote"),
                text!("quote."),
                Inline::Emphasis(vec![text!("second")]),
                text!("line."),
            }]
        );
    }

    #[test]
    fn test_codeblock() {
        assert_parse!(
            "```haskell\nmain: IO ()\n```\n",
            vec![codeblock!("haskell", "main: IO ()\n"),]
        );
        assert_parse!(
            "```\nmain: IO ()\n```\n",
            vec![codeblock!("main: IO ()\n"),]
        );
        assert_parse!(
            r#"# Code
## Haskell code
```haskell
main: IO ()
main = do
    Hello
```
## C code

```c
int main(){{}}
```


## something code

```
fn main(){{}}```
"#,
            vec![
                Block::Heading(1, vec![text!("Code")]),
                Block::Heading(2, vec![text!("Haskell"), text!("code")]),
                codeblock!("haskell", "main: IO ()\nmain = do\n    Hello\n"),
                Block::Heading(2, vec![text!("C"), text!("code")]),
                codeblock!("c", "int main(){{}}\n"),
                Block::Heading(2, vec![text!("something"), text!("code")]),
                codeblock!("fn main(){{}}"),
            ]
        );
    }

    #[test]
    fn test_list() {
        assert_parse!(
            "- a\n- b\n- c\n",
            vec![listblock! {
                ListOrderType::Unordered;
                (None, vec![text!("a")], None),
                (None, vec![text!("b")], None),
                (None, vec![text!("c")], None),
            }]
        );
        assert_parse!(
            "1. one\n2. two\n\n1. 壱\n  a. い\n  b. ろ\n1. 弐\n",
            vec![
                listblock! {
                    ListOrderType::OrderedNumbers;
                    (None, vec![text!("one")], None),
                    (None, vec![text!("two")], None),
                },
                listblock! {
                    ListOrderType::OrderedNumbers;
                    (
                        None,
                        vec![text!("壱")],
                        Some(list! {
                            ListOrderType::OrderedAlphabets;
                            (None, vec![text!("い")], None),
                            (None, vec![text!("ろ")], None),
                        })
                    ),
                    (None, vec![text!("弐")], None),
                },
            ]
        );
        assert_parse!(
            "- a\n- b\n- c\n  - d\n  - e\n\n",
            vec![listblock! {
                ListOrderType::Unordered;
                (None, vec![text!("a")], None),
                (None, vec![text!("b")], None),
                (
                    None,
                    vec![text!("c")],
                    Some(list! {
                        ListOrderType::Unordered;
                        (None, vec![text!("d")], None),
                        (None, vec![text!("e")], None),
                    })
                )
            }]
        );
        assert_parse!(
            "- a\n  - b\n   - c\n",
            vec![listblock! {
                ListOrderType::Unordered;
                (
                    None,
                    vec![text!("a")],
                    Some(list! {
                        ListOrderType::Unordered;
                        (
                            None,
                            vec![text!("b")],
                            Some(list! {
                                ListOrderType::Unordered;
                                (None, vec![text!("c")], None)
                            })
                        ),
                    })
                )
            }]
        );
    }

    #[test]
    fn test_task_list() {
        assert_parse!(
            "- [ ] TODO\n- [x] DONE\n- grouped:\n  1. [ ] todotodo.\n",
            vec![listblock! {
                ListOrderType::Unordered;
                (Some(false), vec![text!("TODO")], None),
                (Some(true), vec![text!("DONE")], None),
                (None, vec![text!("grouped:")], Some(
                    list! {
                        ListOrderType::OrderedNumbers;
                        (Some(false), vec![text!("todotodo.")], None)
                    }
                )),
            }]
        );
    }

    #[test]
    fn test_hr() {
        assert_parse!("\n\n---\n", vec![Block::HorizontalRule]);
        assert_parse!(
            "\n---\n---\n",
            vec![Block::HorizontalRule, Block::HorizontalRule]
        );
    }

    #[test]
    fn test_table() {
        assert_parse!(
            r#"
| A |
|---|
| a |
"#,
            vec![Block::Table(
                vec![Align::Left],
                vec![vec![vec![text!("A")]], vec![vec![text!("a")]]],
                true,
            )]
        );
        assert_parse!(
            r#"
| A |
| - |
| a |
"#,
            vec![Block::Table(
                vec![Align::Left],
                vec![vec![vec![text!("A")]], vec![vec![text!("a")]]],
                true,
            )]
        );
        assert_parse!(
            r#"
|  A |
| :-:|
|  a |
"#,
            vec![Block::Table(
                vec![Align::Center],
                vec![vec![vec![text!("A")]], vec![vec![text!("a")]]],
                true,
            )]
        );
        assert_parse!(
            r#"
| A | B | C | D |     E      |
|:-:|--:| - |---| :--------- |
|1|2|3|4|   5|
"#,
            vec![Block::Table(
                vec![
                    Align::Center,
                    Align::Right,
                    Align::Left,
                    Align::Left,
                    Align::Left,
                ],
                vec![
                    vec![
                        vec![text!("A")],
                        vec![text!("B")],
                        vec![text!("C")],
                        vec![text!("D")],
                        vec![text!("E")],
                    ],
                    vec![
                        vec![text!("1")],
                        vec![text!("2")],
                        vec![text!("3")],
                        vec![text!("4")],
                        vec![text!("5")],
                    ],
                ],
                true,
            )]
        );
    }

    #[test]
    fn test_raw_html() {
        assert_parse!(
            "<span>text</span>\n",
            vec![p! {
                text!("<span>text</span>"),
            }]
        );
    }

    #[test]
    fn test_comment() {
        assert_parse!(
            "My secret key is <!-- i*Love*Rust -->.\n",
            vec![p! {
                text!("My"),
                text!("secret"),
                text!("key"),
                text!("is"),
                comment!(" i*Love*Rust "),
                text!("."),
            }]
        );
    }

    #[test]
    fn test_nested_text() {
        assert_parse!(
            "[*text*](link)\n",
            vec![p! {
                Inline::Link(
                    vec![
                        Inline::Emphasis(vec![text!("text")]),
                    ],
                    String::from("link"),
                ),
            }]
        );
        assert_parse!(
            "[![](image)](link)\n",
            vec![p! {
                Inline::Link(
                    vec![Inline::Image(String::new(), String::from("image"))],
                    String::from("link"),
                ),
            }]
        );
        assert_parse!(
            "__ *a* __\n",
            vec![p! {
                Inline::Strong(vec![
                    Inline::Emphasis(vec![text!("a")]),
                ]),
            }]
        );
    }

    #[test]
    fn test_import() {
        assert_parse!(
            "@(another.md)\n",
            vec![Block::Import(String::from("another.md"))]
        );
        assert_parse!(
            "# h1\n@(another.md)\n",
            vec![
                Block::Heading(1, vec![text!("h1")]),
                Block::Import(String::from("another.md"))
            ]
        );
    }

    #[test]
    fn test_hyperlink_block() {
        assert_parse!("{{url}}\n", vec![Block::HyperLink(String::from("url"))]);
        assert_parse!("{{ url }}\n", vec![Block::HyperLink(String::from("url"))]);
    }

    #[test]
    fn test_code_import() {
        assert_parse!(
            "@[rust](main.rs)\n",
            vec![Block::CodeImport(
                Some(String::from("rust")),
                String::from("main.rs")
            )]
        );
        assert_parse!(
            "@[](main.rs)\n",
            vec![Block::CodeImport(None, String::from("main.rs"))]
        );
    }

    #[test]
    fn test_mathjax() {
        assert_parse!(
            "$$f(x) = x^2.$$\n",
            vec![Block::MathJax(String::from("f(x) = x^2."))]
        );
        assert_parse!(
            "$f(x) = x^2$\n",
            vec![p! {Inline::MathJax(String::from("f(x) = x^2"))}]
        );
        assert_parse!(
            "From $f(x) = x^2$.\n",
            vec![p! {
                text!("From"),
                Inline::MathJax(String::from("f(x) = x^2")),
                text!(".")
            }]
        );
        assert_parse!("$$\\$$$\n", vec![Block::MathJax(String::from("\\$"))]);
        assert_parse!(
            "$\\mu\\$$\n",
            vec![p! {Inline::MathJax(String::from("\\mu\\$"))}]
        );
    }
}
