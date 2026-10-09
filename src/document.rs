// https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf
//
// Unlike the Wadler's algorithm or some other formatters like prettier, we do
// not need to search the best format given source codes. For example, we do
// not have any "group" combinator.
//
// However, we are rather given the "best" format by all information available
// in the source codes like Go.
//
// We need soft-line and if-break nodes to make nodes totally agnostic about if
// parent nodes are broken or not. But that also makes IR more complex.
// (e.g. handling trailing commas in function calls)

use crate::Builder;
use allocator_api2::{
    alloc::{Allocator, Global},
    boxed::Box,
};
use core::ops::Deref;

/// A document.
#[derive(Clone, Debug)]
pub enum Document<'a, A: Allocator = Global> {
    /// A document broken into multiple lines.
    Break {
        broken: bool,
        document: Box<Self, A>,
    },
    /// An indented document.
    Indent(Box<Self, A>),
    /// A line.
    ///
    /// A formatter considers it as a space if a document is not broken by
    /// [`Break`](Document::Break).
    Line,
    /// A line suffix.
    LineSuffix(Str<'a, A>),
    /// A document indented to a current column.
    ///
    /// If it is `soft`, an indent becomes equal to or more than a current
    /// indent.
    Offside { document: Box<Self, A>, soft: bool },
    /// A sequence of documents.
    Sequence(Box<[Self], A>),
    /// A string.
    String(Str<'a, A>),
}

impl<A: Allocator> PartialEq for Document<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Break { broken, document },
                Self::Break {
                    broken: other_broken,
                    document: other_document,
                },
            ) => broken == other_broken && document == other_document,
            (Self::Indent(document), Self::Indent(other)) => document == other,
            (Self::Line, Self::Line) => true,
            (Self::LineSuffix(string), Self::LineSuffix(other))
            | (Self::String(string), Self::String(other)) => string == other,
            (
                Self::Offside { document, soft },
                Self::Offside {
                    document: other_document,
                    soft: other_soft,
                },
            ) => document == other_document && soft == other_soft,
            (Self::Sequence(documents), Self::Sequence(other)) => documents == other,
            _ => false,
        }
    }
}

impl<'a, A: Allocator> From<&'a str> for Document<'a, A> {
    fn from(string: &'a str) -> Self {
        Self::String(Str::Borrowed(string))
    }
}

impl<A: Allocator> From<Box<[Self], A>> for Document<'_, A> {
    fn from(documents: Box<[Self], A>) -> Self {
        Self::Sequence(documents)
    }
}

/// A string in a document.
#[derive(Debug)]
pub enum Str<'a, A: Allocator = Global> {
    /// A borrowed string.
    Borrowed(&'a str),
    /// An owned string.
    Owned(Box<str, A>),
}

impl<A: Allocator + Clone> Clone for Str<'_, A> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(string) => Self::Borrowed(string),
            Self::Owned(string) => {
                Self::Owned(Builder::new(Box::allocator(string).clone()).allocate_str([&**string]))
            }
        }
    }
}

impl<A: Allocator> Deref for Str<'_, A> {
    type Target = str;

    fn deref(&self) -> &str {
        match self {
            Self::Borrowed(string) => string,
            Self::Owned(string) => string,
        }
    }
}

impl<A: Allocator> PartialEq for Str<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{r#break, flatten, indent, line, line_suffix, offside, sequence};

    #[test]
    fn compare_documents() {
        let documents: &[Document] = &[
            r#break(Box::new("foo".into())),
            flatten(Box::new("foo".into())),
            flatten(Box::new("bar".into())),
            indent(Box::new("foo".into())),
            indent(Box::new("bar".into())),
            line(),
            line_suffix("foo"),
            line_suffix("bar"),
            offside(Box::new("foo".into()), false),
            offside(Box::new("foo".into()), true),
            offside(Box::new("bar".into()), false),
            sequence(Box::from(["foo".into()])),
            sequence(Box::from(["bar".into()])),
            "foo".into(),
            "bar".into(),
        ];

        for (index, document) in documents.iter().enumerate() {
            for (other_index, other) in documents.iter().enumerate() {
                assert_eq!(document == other, index == other_index);
            }
        }
    }

    #[test]
    fn compare_borrowed_and_owned_strings() {
        assert_eq!(Str::<Global>::Borrowed("foo"), Str::Owned(Box::from("foo")));
        assert_ne!(Str::<Global>::Borrowed("foo"), Str::Owned(Box::from("bar")));
    }

    #[test]
    fn clone_owned_string() {
        let string = Str::<Global>::Owned(Box::from("foo"));

        assert_eq!(string.clone(), string);
    }
}
