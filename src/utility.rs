//! Utilities.

use super::Document;

/// Checks if a document is broken into multiple lines.
pub fn is_broken(document: &Document) -> bool {
    match document {
        Document::Break(broken, _) => *broken,
        Document::Indent(document) | Document::Offside(document) => is_broken(document),
        Document::Sequence(documents) => documents.iter().any(is_broken),
        Document::Line | Document::LineSuffix(_) | Document::String(_) => false,
    }
}

/// Counts lines in a document.
pub fn count_lines(document: &Document) -> usize {
    match document {
        Document::Break(broken, document) => {
            if *broken {
                count_lines(document)
            } else {
                0
            }
        }
        Document::Indent(document) | Document::Offside(document) => count_lines(document),
        Document::Line => 1,
        Document::Sequence(documents) => documents.iter().map(count_lines).sum(),
        Document::LineSuffix(_) | Document::String(_) => 0,
    }
}

/// Checks if a document is empty.
pub fn is_empty(document: &Document) -> bool {
    match document {
        Document::Break(_, document) | Document::Indent(document) | Document::Offside(document) => {
            is_empty(document)
        }
        Document::Sequence(documents) => documents.iter().all(is_empty),
        Document::LineSuffix(string) | Document::String(string) => string.is_empty(),
        Document::Line => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{flatten, indent, line, line_suffix, r#break};

    #[test]
    fn check_empty() {
        assert!(is_empty(&"".into()));
        assert!(!is_empty(&"foo".into()));
        assert!(!is_empty(&line()));
        assert!(is_empty(&line_suffix("")));
        assert!(!is_empty(&line_suffix("foo")));
        assert!(is_empty(&indent(&"".into())));
        assert!(!is_empty(&indent(&"foo".into())));
        assert!(is_empty(&r#break(&"".into())));
        assert!(!is_empty(&r#break(&"foo".into())));
    }

    #[test]
    fn check_break() {
        assert!(!is_broken(&"".into()));
        assert!(!is_broken(&"foo".into()));
        assert!(!is_broken(&line()));
        assert!(!is_broken(&line_suffix("foo")));
        assert!(!is_broken(&indent(&"foo".into())));
        assert!(is_broken(&r#break(&"".into())));
        assert!(!is_broken(&flatten(&"".into())));
        assert!(!is_broken(&flatten(&r#break(&"".into()))));
    }
}
