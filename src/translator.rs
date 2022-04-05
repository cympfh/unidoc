use crate::entity::{Align, Block, Inline, List, ListItem, ListOrderType, Markdown, Text};
use crate::html;
use crate::html::{Html, HtmlDoc};
use std::collections::VecDeque;

pub struct Translator {
    compact: bool,
    indent: usize,
}

impl Translator {
    pub fn new(compact: bool, indent: usize) -> Self {
        Self { compact, indent }
    }

    /// Returns: (title, body)
    pub fn markdown(&self, doc: &Markdown) -> (String, String) {
        let title = inner_text(&doc[0]);
        let doc: HtmlDoc = doc.iter().map(|md| self.block(md)).collect();
        let body = if self.compact {
            self.show_compact(&doc)
        } else {
            self.show_pretty(&doc)
        };
        (title, body)
    }

    fn show_compact(&self, doc: &HtmlDoc) -> String {
        let mut lines: Vec<String> = vec![];
        let mut queue = VecDeque::new();
        for block in doc.iter() {
            queue.push_back(block);
        }
        while let Some(block) = queue.pop_front() {
            match block {
                Html::Line(line) => lines.push(line.to_string()),
                Html::Block(doc) => {
                    for child in doc.iter().rev() {
                        queue.push_front(child);
                    }
                }
                _ => {}
            }
        }
        lines.join("") + "\n"
    }

    fn show_pretty(&self, doc: &HtmlDoc) -> String {
        let mut indent = 0;
        let mut tab = String::new();
        let mut lines: Vec<String> = vec![];
        let mut queue = VecDeque::new();
        for block in doc.iter() {
            queue.push_back(block);
        }
        while let Some(block) = queue.pop_front() {
            match block {
                Html::Line(line) => lines.push(format!("{}{}", tab, line)),
                Html::Block(doc) => {
                    assert!(doc.len() >= 2);
                    queue.push_front(&doc[doc.len() - 1]);
                    queue.push_front(&Html::Deindent);
                    for i in (1..doc.len() - 1).rev() {
                        queue.push_front(&doc[i]);
                    }
                    queue.push_front(&Html::Indent);
                    queue.push_front(&doc[0]);
                }
                Html::Indent => {
                    indent += self.indent;
                    tab += "  ";
                }
                Html::Deindent => {
                    indent -= self.indent;
                    tab = (0..indent).map(|_| ' ').collect();
                }
            }
        }
        lines.join("\n") + "\n"
    }

    fn block(&self, block: &Block) -> Html {
        match block {
            Block::Heading(level, label) => {
                html![format!("<h{}>{}</h{}>", level, self.text(&label), level)]
            }
            Block::Paragraph(text) => {
                html![format!("<p>{}</p>", self.text(&text))]
            }
            Block::Quoted(text) => {
                html![format!("<q>{}</q>", self.text(&text))]
            }
            Block::Code(language, code) => {
                let class = if let Some(lang) = language {
                    format!("code {}", lang)
                } else {
                    format!("code")
                };
                html![format!(
                    "<pre><code class=\"{}\">{}</code></pre>",
                    class, code
                )]
            }
            Block::HorizontalRule => html![format!("<hr />")],
            Block::ListBlock(list) => self.list(&list),
            Block::Table(aligns, content, has_header) => self.table(&aligns, content, *has_header),
        }
    }

    fn list(&self, list: &List) -> Html {
        let List { order_type, items } = list;
        let (begin, end) = match order_type {
            ListOrderType::Unordered => ("<ul>", "</ul>"),
            ListOrderType::OrderedNumbers => ("<ol>", "</ol>"),
            ListOrderType::OrderedAlphabets => ("<ol type=a>", "</ol>"),
        };
        let mut lines = vec![html![begin.to_string()]];
        for item in items.iter() {
            lines.push(self.listitem(item));
        }
        lines.push(html![end.to_string()]);
        Html::Block(lines)
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
        let li = html![format!("<li>{}{}</li>", checkbox, self.text(label))];
        if let Some(children) = children {
            html![li, self.list(children)]
        } else {
            li
        }
    }

    fn table(&self, aligns: &Vec<Align>, content: &Vec<Vec<Text>>, has_header: bool) -> Html {
        let mut lines = vec![];
        lines.push(html![format!("<table>")]);
        // thead
        if has_header {
            let mut tr = vec![html![format!("<tr class=header>")]];
            for (t, align) in content[0].iter().zip(aligns.iter()) {
                tr.push(self.cell(t, align, true));
            }
            tr.push(html![format!("</tr>")]);
            lines.push(html![
                html![format!("<thead>")],
                Html::Block(tr),
                html![format!("</thead>")],
            ]);
        }
        // tbody
        let start = if has_header { 1 } else { 0 };
        let mut tbody = vec![html![format!("<tbody>")]];
        for i in start..content.len() {
            let class = if (i - start) % 2 == 0 { "odd" } else { "even" };
            let mut tr = vec![html![format!("<tr class={}>", class)]];
            for (t, align) in content[i].iter().zip(aligns.iter()) {
                tr.push(self.cell(t, align, false));
            }
            tr.push(html![format!("</tr>")]);
            tbody.push(Html::Block(tr));
        }
        tbody.push(html![format!("</tbody>")]);
        lines.push(Html::Block(tbody));
        lines.push(html![format!("</table>")]);
        Html::Block(lines)
    }

    fn cell(&self, text: &Text, align: &Align, is_header: bool) -> Html {
        let tag = if is_header { "th" } else { "td" };
        let html = match align {
            Align::Left => format!("<{} align=left>{}</{}>", tag, self.text(text), tag),
            Align::Right => format!("<{} align=right>{}</{}>", tag, self.text(text), tag),
            _ => format!("<{}>{}</{}>", tag, self.text(text), tag),
        };
        Html::Line(html)
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
            Inline::Image(alt, image) => format!("<img src=\"{}\" alt=\"{}\" />", image, alt),
            Inline::Code(text) => format!("<code>{}</code>", text),
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
        text.iter().map(from_inline).collect::<Vec<_>>().join(" ")
    }
    fn from_inline(inline: &Inline) -> String {
        match inline {
            Inline::Link(text, _) => from_text(text),
            Inline::Image(alt, _) => alt.to_string(),
            Inline::Code(text) => text.to_string(),
            Inline::Emphasis(text) => from_text(text),
            Inline::Strong(text) => from_text(text),
            Inline::EmphasisAndStrong(text) => from_text(text),
            Inline::Deleted(text) => from_text(text),
            Inline::Plaintext(text) => text.to_string(),
            _ => String::new(),
        }
    }
    match block {
        Block::Heading(_, label) => from_text(label),
        Block::Paragraph(text) => from_text(text),
        Block::Quoted(text) => from_text(text),
        _ => String::new(),
    }
}
