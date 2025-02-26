use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

use crate::blogcard::blogcard;
use crate::entity::html::{Html, HtmlDoc};
use crate::entity::markdown::{
    Align, Block, Inline, List, ListItem, ListOrderType, Markdown, Text,
};
use crate::executor::Executor;
use crate::io;
use crate::parser;
use crate::webpage::WebPage;
use crate::{leaf, node};
use std::path::Path;

pub struct Translator {
    filedir: Option<String>,
}

impl Translator {
    pub fn new(filedir: Option<String>) -> Self {
        Self { filedir }
    }

    /// Returns: (title, body)
    pub fn markdown(&self, mkd: &Markdown) -> HtmlDoc {
        let title = inner_text(&mkd[0]);
        HtmlDoc::new(title, mkd.iter().map(|md| self.block(md)).collect())
    }

    fn block(&self, block: &Block) -> Html {
        match block {
            Block::Heading(1, label) => {
                let innerhtml = self.text(&label);
                let id = format!("{}-{}", 1, percent_encode(&innerhtml));
                leaf!("<h1 class=\"title\" id=\"{}\">{}</h1>", id, innerhtml)
            }
            Block::Heading(level, label) => {
                let innerhtml = self.text(&label);
                let id = format!("{}-{}", level, percent_encode(&innerhtml));
                leaf!("<h{} id=\"{}\">{}</h{}>", level, id, innerhtml, level)
            }
            Block::Paragraph(text) => {
                leaf!("<p>{}</p>", self.text(&text))
            }
            Block::Quoted(text) => {
                leaf!("<blockquote>{}</blockquote>", self.text(&text))
            }
            Block::Code(language, code) => {
                // executor check
                match language.clone() {
                    Some(x) if x == String::from("@bash") => {
                        let res = Executor::bash(code);
                        if res.is_ok() {
                            leaf!("<pre><code>{}</code></pre>", res.unwrap())
                        } else {
                            leaf!("<pre><samp class=error>{}</samp></pre>", res.unwrap())
                        }
                    }
                    Some(x) if x == String::from("@dot") || x == String::from("@graphviz") => {
                        let res = Executor::dot(code);
                        if res.is_ok() {
                            leaf!("<img src=\"{}\">", res.unwrap())
                        } else {
                            leaf!("<pre><samp class=error>{}</samp></pre>", res.unwrap())
                        }
                    }
                    Some(x) if x == String::from("@gnuplot") => {
                        let res = Executor::gnuplot(code);
                        if res.is_ok() {
                            leaf!("{}", res.unwrap())
                        } else {
                            leaf!("<pre><samp class=error>{}</samp></pre>", res.unwrap())
                        }
                    }
                    _ => {
                        // simple code block
                        let class = if let Some(lang) = language {
                            format!("code language-{}", lang)
                        } else {
                            format!("code")
                        };
                        leaf!(
                            "<pre><code class=\"{}\">{}</code></pre>",
                            class,
                            encode(&code)
                        )
                    }
                }
            }
            Block::HorizontalRule => leaf!("<hr />"),
            Block::ListBlock(list) => self.list(&list),
            Block::Table(aligns, content, has_header) => self.table(&aligns, content, *has_header),
            Block::Import(path) => {
                if let Some(path) = find(&path, &self.filedir) {
                    let content = io::read(&path.to_string()).unwrap();
                    let mkd = parser::markdown(&content).unwrap();
                    let doc = self.markdown(&mkd);
                    doc.as_html()
                } else {
                    panic!("Cannot find {}", path);
                }
            }
            Block::HyperLink(url) => blogcard(url.to_string()),
            Block::CodeImport(language, path) => {
                if let Some(path) = find(&path, &self.filedir) {
                    let content = io::read(&path.to_string()).unwrap();
                    let codeblock = Block::Code(language.clone(), content.to_string());
                    self.block(&codeblock)
                } else {
                    panic!("Cannot find {}", path);
                }
            }
            Block::MathJax(tex) => {
                leaf!("\\[{}\\]", encode(tex))
            }
        }
    }

    fn list(&self, list: &List) -> Html {
        let List { order_type, items } = list;
        let (begin, end) = match order_type {
            ListOrderType::Unordered => ("<ul>", "</ul>"),
            ListOrderType::OrderedNumbers => ("<ol>", "</ol>"),
            ListOrderType::OrderedAlphabets => ("<ol type=a>", "</ol>"),
        };
        let mut html = node!(leaf!(begin), leaf!(end));
        for item in items.iter() {
            html.push(self.listitem(item));
        }
        html
    }

    fn listitem(&self, listitem: &ListItem) -> Html {
        let ListItem {
            checked,
            label,
            children,
        } = listitem;
        let checkbox = if let Some(checked) = checked {
            if *checked {
                "<input type=checkbox checked=checked disabled=disabled>"
            } else {
                "<input type=checkbox disabled=disabled>"
            }
        } else {
            ""
        };
        if let Some(children) = children {
            let mut html = node!(leaf!("<li>"), leaf!("</li>"));
            if checked.is_some() {
                html.push(leaf!(checkbox));
            }
            html.push(leaf!(self.text(label)));
            html.push(self.list(children));
            html
        } else {
            leaf!("<li>{}{}</li>", checkbox, self.text(label))
        }
    }

    fn table(&self, aligns: &Vec<Align>, content: &Vec<Vec<Text>>, has_header: bool) -> Html {
        let mut html = node!(leaf!("<table>"), leaf!("</table>"));
        // thead
        if has_header {
            let mut thead = node!(leaf!("<thead>"), leaf!("</thead>"));
            let mut tr = node!(leaf!("<tr class=header>"), leaf!("</tr>"));
            for (t, align) in content[0].iter().zip(aligns.iter()) {
                tr.push(leaf!(self.cell(t, align, true)));
            }
            thead.push(tr);
            html.push(thead);
        }
        // tbody
        let start = if has_header { 1 } else { 0 };
        let mut tbody = node!(leaf!("<tbody>"), leaf!("</tbody>"));
        for i in start..content.len() {
            let class = if (i - start) % 2 == 0 { "odd" } else { "even" };
            let mut tr = node![leaf!("<tr class={}>", class), leaf!("</tr>")];
            for (t, align) in content[i].iter().zip(aligns.iter()) {
                tr.push(leaf!(self.cell(t, align, false)));
            }
            tbody.push(tr);
        }
        html.push(tbody);
        html
    }

    fn cell(&self, text: &Text, align: &Align, is_header: bool) -> String {
        let tag = if is_header { "th" } else { "td" };
        match align {
            Align::Left => format!("<{} align=left>{}</{}>", tag, self.text(text), tag),
            Align::Right => format!("<{} align=right>{}</{}>", tag, self.text(text), tag),
            Align::Center => format!("<{} align=center>{}</{}>", tag, self.text(text), tag),
        }
    }

    fn text(&self, text: &Text) -> String {
        text.iter()
            .map(|s| self.inline(s))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn inline(&self, inline: &Inline) -> String {
        match inline {
            Inline::Link(text, url) => format!("<a href=\"{}\">{}</a>", url, self.text(text)),
            Inline::HyperLink(url) => {
                if let Some(title) = WebPage::new(url.to_string()).title() {
                    format!("<a href=\"{}\">{}</a>", url, encode(&title))
                } else {
                    format!("<a href=\"{}\">{}</a>", url, url)
                }
            }
            Inline::Image(alt, image) => format!("<img src=\"{}\" alt=\"{}\" />", image, alt),
            Inline::Code(text) => format!("<code>{}</code>", encode(text)),
            Inline::Emphasis(text) => format!("<em>{}</em>", self.text(text)),
            Inline::Strong(text) => format!("<strong>{}</strong>", self.text(text)),
            Inline::EmphasisAndStrong(text) => {
                format!("<em><strong>{}</strong></em>", self.text(text))
            }
            Inline::Deleted(text) => format!("<del>{}</del>", self.text(text)),
            Inline::Plaintext(text) => text.to_string(),
            Inline::Newline => format!("<br />"),
            Inline::Comment(text) => format!("<!--{}-->", text),
            Inline::MathJax(tex) => format!("\\({}\\)", encode(tex)),
            Inline::Emoji(shortcode) => {
                if let Some(emoji) = emojis::get_by_shortcode(&shortcode) {
                    emoji.to_string()
                } else {
                    // fail-over
                    format!(":{}:", shortcode)
                }
            }
        }
    }
}

fn inner_text(block: &Block) -> String {
    fn from_text(text: &Text) -> String {
        text.iter()
            .map(from_inline)
            .flatten()
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn from_inline(inline: &Inline) -> Option<String> {
        match inline {
            Inline::Link(text, _) => Some(from_text(text)),
            Inline::Image(alt, _) => Some(encode(alt)),
            Inline::Code(text) => Some(encode(text)),
            Inline::Emphasis(text) => Some(from_text(text)),
            Inline::Strong(text) => Some(from_text(text)),
            Inline::EmphasisAndStrong(text) => Some(from_text(text)),
            Inline::Deleted(text) => Some(from_text(text)),
            Inline::Plaintext(text) => Some(encode(text)),
            Inline::HyperLink(url) => Some(url.to_string()),
            Inline::Newline => None,
            Inline::Comment(_) => None,
            Inline::MathJax(tex) => Some(encode(tex)),
            Inline::Emoji(shortcode) => Some(shortcode.to_string()),
        }
    }
    match block {
        Block::Heading(_, label) => from_text(label),
        Block::Paragraph(text) => from_text(text),
        Block::Quoted(text) => from_text(text),
        _ => String::new(),
    }
}

fn find(path: &String, filedir: &Option<String>) -> Option<String> {
    let f = Path::new(path);
    if f.is_file() {
        return Some(path.to_string());
    }
    if f.is_relative() {
        if let Some(dir) = filedir {
            let f = Path::new(&dir).join(f);
            if f.is_file() {
                return Some(String::from(f.to_str().unwrap()));
            }
        }
    }
    None
}

fn encode(html: &String) -> String {
    html_escape::encode_safe(html).to_string()
}

fn percent_encode(input: &String) -> String {
    const CUSTOM_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'_');
    utf8_percent_encode(&input, &CUSTOM_ENCODE_SET).to_string()
}

#[cfg(test)]
mod test_translator {

    use crate::translator::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode(&"f(x) < g(x) > 1".to_string()),
            "f(x) &lt; g(x) &gt; 1".to_string()
        );
    }
}
