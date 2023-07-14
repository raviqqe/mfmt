use super::{utility::is_broken, Document};

pub fn sequence<'a>(documents: &[Document<'a>]) -> Document<'a> {
    Document::Sequence(documents)
}

pub fn line_suffix<'a>(string: &'a str) -> Document<'a> {
    Document::LineSuffix(string.into())
}

pub fn flatten<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(false, document)
}

pub fn r#break<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(true, document)
}

pub fn flatten_if<'a>(condition: bool, document: &'a Document<'a>) -> Document<'a> {
    Document::Break(!condition || is_broken(&document), document)
}

pub fn indent<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Indent(document)
}

pub const fn line() -> Document<'static> {
    Document::Line
}

pub fn empty() -> Document<'static> {
    "".into()
}
