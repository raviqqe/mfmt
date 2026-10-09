//! Document builders.

mod builder;

use super::{Document, Str, utility::is_broken};
use allocator_api2::{alloc::Allocator, boxed::Box};
pub use builder::Builder;

/// Creates a sequence of documents.
pub const fn sequence<'a, A: Allocator>(documents: Box<[Document<'a, A>], A>) -> Document<'a, A> {
    Document::Sequence(documents)
}

/// Creates a line suffix.
pub const fn line_suffix<A: Allocator>(string: &str) -> Document<'_, A> {
    Document::LineSuffix(Str::Borrowed(string))
}

/// Flattens a document.
pub const fn flatten<'a, A: Allocator>(document: Box<Document<'a, A>, A>) -> Document<'a, A> {
    Document::Break {
        broken: false,
        document,
    }
}

/// Breaks a document into multiple lines.
pub const fn r#break<'a, A: Allocator>(document: Box<Document<'a, A>, A>) -> Document<'a, A> {
    Document::Break {
        broken: true,
        document,
    }
}

/// Flattens a document if a `condition` is true.
pub fn flatten_if<'a, A: Allocator>(
    condition: bool,
    document: Box<Document<'a, A>, A>,
) -> Document<'a, A> {
    Document::Break {
        broken: !condition || is_broken(&document),
        document,
    }
}

/// Indents a document.
pub const fn indent<'a, A: Allocator>(document: Box<Document<'a, A>, A>) -> Document<'a, A> {
    Document::Indent(document)
}

/// Creates a new line.
pub const fn line<A: Allocator>() -> Document<'static, A> {
    Document::Line
}

/// Creates an empty document.
pub const fn empty<A: Allocator>() -> Document<'static, A> {
    Document::String(Str::Borrowed(""))
}

/// Creates a document indented to a current column.
pub const fn offside<'a, A: Allocator>(
    document: Box<Document<'a, A>, A>,
    soft: bool,
) -> Document<'a, A> {
    Document::Offside { document, soft }
}
