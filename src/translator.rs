use crate::blogcard::blogcard;
use crate::entity::html::{Html, HtmlDoc};
use crate::entity::markdown::{
    Align, Block, Inline, List, ListItem, ListOrderType, Markdown, Text,
};
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
    pub fn markdown(&self, mkd: &Markdown) -> (String, HtmlDoc) {
        let title = inner_text(&mkd[0]);
        let body = HtmlDoc::new(mkd.iter().map(|md| self.block(md)).collect());
        (title, body)
    }

    fn block(&self, block: &Block) -> Html {
        match block {
            Block::Heading(level, label) => {
                leaf!("<h{}>{}</h{}>", level, self.text(&label), level)
            }
            Block::Paragraph(text) => {
                leaf!("<p>{}</p>", self.text(&text))
            }
            Block::Quoted(text) => {
                leaf!("<q>{}</q>", self.text(&text))
            }
            Block::Code(language, code) => {
                let class = if let Some(lang) = language {
                    format!("code {}", lang)
                } else {
                    format!("code")
                };
                leaf!("<pre><code class=\"{}\">{}</code></pre>", class, code)
            }
            Block::HorizontalRule => leaf!("<hr />"),
            Block::ListBlock(list) => self.list(&list),
            Block::Table(aligns, content, has_header) => self.table(&aligns, content, *has_header),
            Block::Import(path) => {
                if let Some(path) = find(&path, &self.filedir) {
                    let content = io::read(&Some(path.to_string())).unwrap();
                    let mkd = parser::markdown(&content).unwrap();
                    let (_, htmldoc) = self.markdown(&mkd);
                    htmldoc.as_html()
                } else {
                    panic!("Cannot find {}", path);
                }
            }
            Block::HyperLink(url) => blogcard(url.to_string()),
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
