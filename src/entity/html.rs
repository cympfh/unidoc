use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlDoc {
    pub title: String,
    doc: Vec<Html>,
}
impl HtmlDoc {
    pub fn new(title: String, doc: Vec<Html>) -> Self {
        Self { title, doc }
    }
    pub fn as_html(self) -> Html {
        Html::Node(
            Box::new(Html::Leaf(String::from("<div>"))),
            self.doc,
            Box::new(Html::Leaf(String::from("</div>"))),
        )
    }
    pub fn append(&mut self, other: &mut HtmlDoc) {
        self.doc.append(&mut other.doc);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Html {
    Leaf(String),
    Node(Box<Html>, Vec<Html>, Box<Html>),
}

#[macro_export]
macro_rules! leaf {
    ( $f:expr ) => { Html::Leaf(String::from($f)) };
    ( $f:expr, $( $xs:expr ),* $(,)? ) => {
        Html::Leaf(format!($f, $($xs),*))
    };
}

#[macro_export]
macro_rules! node {
    ( $begin:expr, $end:expr ) => {
        Html::Node(Box::new($begin), vec![], Box::new($end))
    };
    ( $begin:expr, $end:expr; [ $($children:expr),* ] ) => {
        Html::Node(Box::new($begin), vec![ $($children),* ], Box::new($end))
    };
}

impl HtmlDoc {
    pub fn show(&self, compact: bool, indent: usize) -> String {
        if compact {
            self.show_compact()
        } else {
            self.show_pretty(indent)
        }
    }

    fn show_compact(&self) -> String {
        let mut lines: Vec<String> = vec![];
        let mut queue = VecDeque::new();
        for block in self.doc.iter() {
            queue.push_back(block);
        }
        while let Some(block) = queue.pop_front() {
            match block {
                Html::Leaf(line) => lines.push(line.to_string()),
                Html::Node(begin, children, end) => {
                    queue.push_front(end);
                    for child in children.iter().rev() {
                        queue.push_front(child);
                    }
                    queue.push_front(begin);
                }
            }
        }
        lines.join("") + "\n"
    }

    fn show_pretty(&self, indentshift: usize) -> String {
        let mut indent = 0;
        let mut tab = String::new();
        let mut lines: Vec<String> = vec![];
        enum Q<'t> {
            Push(&'t Html),
            Indent,
            Deindent,
        }
        let mut queue = VecDeque::new();
        for block in self.doc.iter() {
            queue.push_back(Q::Push(&block));
        }
        while let Some(block) = queue.pop_front() {
            match block {
                Q::Push(Html::Leaf(leaf)) => lines.push(format!("{}{}", tab, leaf)),
                Q::Push(Html::Node(begin, doc, end)) => {
                    queue.push_front(Q::Push(&end));
                    queue.push_front(Q::Deindent);
                    for child in doc.iter().rev() {
                        queue.push_front(Q::Push(&child));
                    }
                    queue.push_front(Q::Indent);
                    queue.push_front(Q::Push(&begin));
                }
                Q::Indent => {
                    indent += indentshift;
                    tab = (0..indent).map(|_| ' ').collect();
                }
                Q::Deindent => {
                    indent -= indentshift;
                    tab = (0..indent).map(|_| ' ').collect();
                }
            }
        }
        lines.join("\n") + "\n"
    }
}

impl Html {
    pub fn push(&mut self, child: Html) {
        if let Html::Node(_, v, _) = self {
            v.push(child);
        }
    }
}

#[cfg(test)]
mod test_main {
    use crate::entity::html::*;
    use crate::{leaf, node};

    #[test]
    fn test_html_push() {
        let mut p = node!(leaf!("<p>"), leaf!("</p>"));
        p.push(leaf!("<img src={} />", "image"));
        assert_eq!(
            p,
            Html::Node(
                Box::new(Html::Leaf(String::from("<p>"))),
                vec![Html::Leaf(String::from("<img src=image />"))],
                Box::new(Html::Leaf(String::from("</p>"))),
            )
        );
    }

    #[test]
    fn test_show() {
        let html = node!(
            leaf!("<body>"),
            leaf!("</body>"); [
                node!(leaf!("<p>"), leaf!("</p>"))
            ]
        );
        let doc = HtmlDoc::new(String::new(), vec![html]);
        let expected = "<body><p></p></body>\n";
        assert_eq!(doc.show_compact(), String::from(expected));
    }
}
