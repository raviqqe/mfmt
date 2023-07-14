use super::{utility::is_broken, Document};

pub fn sequence<'a, D: Into<Document<'a>>>(iterator: impl IntoIterator<Item = D>) -> Document<'a> {
    Document::Sequence(
        iterator
            .into_iter()
            .map(|document| document.into())
            .collect(),
    )
}

pub fn line_suffix<'a>(string: &'a str) -> Document<'a> {
    Document::LineSuffix(string.into())
}

pub fn flatten<'a>(document: impl Into<Document<'a>>) -> Document<'a> {
    Document::Break(false, document.into().into())
}

pub fn r#break<'a>(document: impl Into<Document<'a>>) -> Document<'a> {
    Document::Break(true, document.into().into())
}

pub fn flatten_if<'a>(condition: bool, document: impl Into<Document<'a>>) -> Document<'a> {
    let document = document.into();

    Document::Break(!condition || is_broken(&document), document.into())
}

pub fn indent<'a>(document: impl Into<Document<'a>>) -> Document<'a> {
    Document::Indent(document.into().into())
}

pub const fn line() -> Document<'static> {
    Document::Line
}

pub fn empty() -> Document<'static> {
    "".into()
}
