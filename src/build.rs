//! Document builders.

mod builder;

use super::{utility::is_broken, Document};
pub use builder::Builder;

/// Creates a sequence of documents.
pub const fn sequence<'a>(documents: &'a [Document<'a>]) -> Document<'a> {
    Document::Sequence(documents)
}

/// Creates a line suffix.
pub const fn line_suffix(string: &str) -> Document<'_> {
    Document::LineSuffix(string)
}

/// Flattens a document.
pub const fn flatten<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(false, document)
}

/// Breaks a document into multiple lines.
pub const fn r#break<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Break(true, document)
}

/// Flattens a document if a `condition` is true.
pub fn flatten_if<'a>(condition: bool, document: &'a Document<'a>) -> Document<'a> {
    Document::Break(!condition || is_broken(document), document)
}

/// Indents a document.
pub const fn indent<'a>(document: &'a Document<'a>) -> Document<'a> {
    Document::Indent(document)
}

/// Creates a new line.
pub const fn line() -> Document<'static> {
    Document::Line
}

/// Creates an empty document.
pub const fn empty() -> Document<'static> {
    Document::String("")
}
