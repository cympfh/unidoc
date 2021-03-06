pub type Markdown = Vec<Block>;
type Url = String;
type FilePath = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Heading(usize, Text),
    ListBlock(List),
    Paragraph(Text),
    Quoted(Text),
    Code(Option<String>, String),
    CodeImport(Option<String>, FilePath),
    HorizontalRule,
    Table(Vec<Align>, Vec<Vec<Text>>, bool), // (Alignments, Content, first_is_header?)
    Import(FilePath),
    HyperLink(Url),
    MathJax(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    pub order_type: ListOrderType,
    pub items: Vec<ListItem>,
}
impl List {
    pub fn new(order_type: ListOrderType, items: Vec<ListItem>) -> Self {
        Self { order_type, items }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ListOrderType {
    Unordered,
    OrderedNumbers,
    OrderedAlphabets,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListItem {
    pub checked: Option<bool>,
    pub label: Text,
    pub children: Option<List>,
}
impl ListItem {
    pub fn new(checked: Option<bool>, label: Text, children: Option<List>) -> Self {
        Self {
            checked,
            label,
            children,
        }
    }
}

pub type Text = Vec<Inline>;

#[derive(Clone, Debug, PartialEq)]
pub enum Inline {
    Link(Text, Url),
    HyperLink(Url),
    Image(String, Url),
    Code(String),
    Emphasis(Text),
    Strong(Text),
    EmphasisAndStrong(Text),
    Deleted(Text),
    Plaintext(String),
    Newline,
    Comment(String),
    MathJax(String),
    Emoji(String),
}
