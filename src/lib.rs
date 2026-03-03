pub mod desktop;
pub mod io;
pub mod parser;
pub mod ui;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub strike: bool,
    pub link: Option<String>,
    pub image: Option<String>,
}

impl Span {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            italic: false,
            code: false,
            strike: false,
            link: None,
            image: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListKind {
    Unordered,
    Ordered { start: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub spans: Vec<Span>,
    pub checked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading {
        level: u8,
        spans: Vec<Span>,
    },
    Paragraph {
        spans: Vec<Span>,
    },
    List {
        kind: ListKind,
        items: Vec<ListItem>,
    },
    Quote {
        blocks: Vec<Block>,
    },
    CodeBlock {
        code: String,
    },
    Divider,
    Table {
        headers: Vec<Vec<Span>>,
        rows: Vec<Vec<Vec<Span>>>,
    },
}
