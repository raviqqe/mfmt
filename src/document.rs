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

/// A document.
#[derive(Clone, Debug, PartialEq)]
pub enum Document<'a> {
    /// A document broken into multiple lines.
    Break(bool, &'a Document<'a>),
    /// An indented document.
    Indent(&'a Document<'a>),
    /// A line.
    ///
    /// A formatter considers it as a space if a document is not broken.
    Line,
    /// A line suffix.
    LineSuffix(&'a str),
    /// A sequence of documents.
    Sequence(&'a [Document<'a>]),
    /// A string.
    String(&'a str),
}

impl<'a> From<&'a str> for Document<'a> {
    fn from(string: &'a str) -> Self {
        Self::String(string)
    }
}

impl<'a> From<&'a [Self]> for Document<'a> {
    fn from(documents: &'a [Self]) -> Self {
        Self::Sequence(documents)
    }
}
