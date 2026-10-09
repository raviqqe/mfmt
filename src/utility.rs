//! Utilities.

use super::Document;
use allocator_api2::alloc::Allocator;

/// Checks if a document is broken into multiple lines.
pub fn is_broken<A: Allocator>(document: &Document<A>) -> bool {
    match document {
        Document::Break { broken, document } => *broken || is_broken(document),
        Document::Indent(document) | Document::Offside { document, .. } => is_broken(document),
        Document::Sequence(documents) => documents.iter().any(is_broken),
        Document::Line | Document::LineSuffix(_) | Document::String(_) => false,
    }
}

/// Counts lines in a document.
pub fn count_lines<A: Allocator>(document: &Document<A>) -> usize {
    match document {
        Document::Break { broken, document } => {
            if *broken {
                count_lines(document)
            } else {
                0
            }
        }
        Document::Indent(document) | Document::Offside { document, .. } => count_lines(document),
        Document::Line => 1,
        Document::Sequence(documents) => documents.iter().map(count_lines).sum(),
        Document::LineSuffix(_) | Document::String(_) => 0,
    }
}

/// Checks if a document is empty.
pub fn is_empty<A: Allocator>(document: &Document<A>) -> bool {
    match document {
        Document::Break { document, .. }
        | Document::Indent(document)
        | Document::Offside { document, .. } => is_empty(document),
        Document::Sequence(documents) => documents.iter().all(is_empty),
        Document::LineSuffix(string) | Document::String(string) => string.is_empty(),
        Document::Line => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{r#break, flatten, indent, line, line_suffix, sequence};
    use allocator_api2::{alloc::Global, boxed::Box};

    #[test]
    fn check_empty() {
        assert!(is_empty::<Global>(&"".into()));
        assert!(!is_empty::<Global>(&"foo".into()));
        assert!(!is_empty::<Global>(&line()));
        assert!(is_empty::<Global>(&line_suffix("")));
        assert!(!is_empty::<Global>(&line_suffix("foo")));
        assert!(is_empty(&indent(Box::new("".into()))));
        assert!(!is_empty(&indent(Box::new("foo".into()))));
        assert!(is_empty(&r#break(Box::new("".into()))));
        assert!(!is_empty(&r#break(Box::new("foo".into()))));
    }

    #[test]
    fn check_break() {
        assert!(!is_broken::<Global>(&"".into()));
        assert!(!is_broken::<Global>(&"foo".into()));
        assert!(!is_broken::<Global>(&line()));
        assert!(!is_broken::<Global>(&line_suffix("foo")));
        assert!(!is_broken(&indent(Box::new("foo".into()))));
        assert!(!is_broken(&flatten(Box::new("".into()))));
        assert!(is_broken(&r#break(Box::new("".into()))));
        assert!(is_broken(&r#break(Box::new(flatten(Box::new("".into()))))));
        assert!(is_broken(&flatten(Box::new(r#break(Box::new("".into()))))));
        assert!(is_broken(&flatten(Box::new(sequence(Box::from([
            r#break(Box::new("".into()))
        ]))))));
    }
}
