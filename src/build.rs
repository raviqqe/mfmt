use super::{utility::is_broken, Document};

pub const fn sequence<'a>(documents: &'a [Document<'a>]) -> Document<'a> {
    Document::Sequence(documents)
}

pub const fn line_suffix<'a>(string: &'a str) -> Document<'a> {
    Document::LineSuffix(string)
}

pub const fn flatten<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(false, document)
}

pub const fn r#break<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(true, document)
}

pub fn flatten_if<'a>(condition: bool, document: &'a Document<'a>) -> Document<'a> {
    Document::Break(!condition || is_broken(&document), document)
}

pub const fn indent<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Indent(document)
}

pub const fn line() -> Document<'static> {
    Document::Line
}

pub const fn empty() -> Document<'static> {
    Document::String("")
}
