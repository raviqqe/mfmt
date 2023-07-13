use super::Document;

pub fn is_broken(document: &Document) -> bool {
    match document {
        Document::Break(broken, _) => *broken,
        Document::Indent(document) => is_broken(document),
        Document::Sequence(documents) => documents.iter().any(is_broken),
        Document::Line | Document::LineSuffix(_) | Document::String(_) => false,
    }
}

pub fn count_lines(document: &Document) -> usize {
    match document {
        Document::Break(broken, document) => {
            if *broken {
                count_lines(document)
            } else {
                0
            }
        }
        Document::Indent(document) => count_lines(document),
        Document::Line => 1,
        Document::Sequence(documents) => documents.iter().map(count_lines).sum(),
        Document::LineSuffix(_) | Document::String(_) => 0,
    }
}

pub fn is_empty(document: &Document) -> bool {
    match document {
        Document::Break(_, document) => is_empty(document),
        Document::Indent(document) => is_empty(document),
        Document::Sequence(documents) => documents.iter().all(is_empty),
        Document::LineSuffix(string) | Document::String(string) => string.is_empty(),
        Document::Line => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{line, line_suffix};

    #[test]
    fn check_empty() {
        assert!(is_empty(&"".into()));
        assert!(!is_empty(&"foo".into()));
        assert!(!is_empty(&line()));
        assert!(is_empty(&line_suffix("")));
        assert!(!is_empty(&line_suffix("foo")));
    }
}
