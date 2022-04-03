use crate::entity::{Align, Block, Inline, List, ListItem, ListOrderType, Markdown, Text};

pub struct Translator {
    compact: bool,
}

impl Translator {
    pub fn new(compact: bool) -> Self {
        Self { compact }
    }

    /// Returns: (title, body)
    pub fn markdown(&self, doc: &Markdown) -> (String, String) {
        let mut html = vec![];
        for block in doc.iter() {
            let b = match block {
                Block::Heading(level, label) => {
                    format!("<h{}>{}</h{}>", level, self.text(&label), level)
                }
                Block::Paragraph(text) => {
                    format!("<p>{}</p>", self.text(&text))
                }
                Block::Quoted(text) => {
                    format!("<q>{}</q>", self.text(&text))
                }
                Block::Code(language, code) => {
                    let class = if let Some(lang) = language {
                        format!("code {}", lang)
                    } else {
                        format!("code")
                    };
                    format!("<pre><code class=\"{}\">{}</code></pre>", class, code)
                }
                Block::HorizontalRule => format!("<hr />"),
                Block::ListBlock(list) => self.list(&list),
                Block::Table(aligns, content, has_header) => {
                    self.table(&aligns, content, *has_header)
                }
            };
            html.push(b);
        }

        let title = inner_text(&doc[0]);
        let mut body = html.join("");
        body.push('\n');

        (title, body)
    }

    fn list(&self, list: &List) -> String {
        let List { order_type, items } = list;
        let (begin, end) = match order_type {
            ListOrderType::Unordered => ("<ul>", "</ul>"),
            ListOrderType::OrderedNumbers => ("<ol>", "</ol>"),
            ListOrderType::OrderedAlphabets => ("<ol type=a>", "</ol>"),
        };
        let mut ret = vec![begin.to_string()];
        for item in items.iter() {
            ret.push(self.listitem(item));
        }
        ret.push(end.to_string());
        ret.join("")
    }

    fn listitem(&self, listitem: &ListItem) -> String {
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
        let children = if let Some(children) = children {
            self.list(children)
        } else {
            String::new()
        };
        format!("<li>{}{}</li>{}", checkbox, self.text(label), children)
    }

    fn table(&self, aligns: &Vec<Align>, content: &Vec<Vec<Text>>, has_header: bool) -> String {
        let mut frags = vec![];
        frags.push(format!("<table>"));
        if has_header {
            frags.push(format!("<thead>"));
            frags.push(format!("<tr class=header>"));
            for (t, align) in content[0].iter().zip(aligns.iter()) {
                frags.push(self.cell(t, align, true));
            }
            frags.push(format!("</tr>"));
            frags.push(format!("</thead>"));
        }
        let start = if has_header { 1 } else { 0 };
        frags.push(format!("<tbody>"));
        for i in start..content.len() {
            let class = if (i - start) % 2 == 0 { "odd" } else { "even" };
            frags.push(format!("<tr class={}>", class));
            for (t, align) in content[i].iter().zip(aligns.iter()) {
                frags.push(self.cell(t, align, false));
            }
            frags.push(format!("</tr>"));
        }
        frags.push(format!("</tbody>"));
        frags.push(format!("</table>"));
        frags.join("")
    }

    fn cell(&self, text: &Text, align: &Align, is_header: bool) -> String {
        let tag = if is_header { "th" } else { "td" };
        match align {
            Align::Left => format!("<{} align=left>{}</{}>", tag, self.text(text), tag),
            Align::Right => format!("<{} align=right>{}</{}>", tag, self.text(text), tag),
            _ => format!("<{}>{}</{}>", tag, self.text(text), tag),
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
