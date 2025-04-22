use super::{Document, r#break, flatten, indent, line_suffix, offside, sequence};
use alloc::{alloc::Allocator, boxed::Box, str, vec::Vec};

/// Document builder.
#[derive(Clone, Debug)]
pub struct Builder<A: Allocator> {
    allocator: A,
}

impl<'a, A: Allocator + Clone + 'a> Builder<A> {
    /// Creates a document builder.
    pub fn new(allocator: A) -> Self {
        Self { allocator }
    }

    /// Returns an allocator.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Breaks a document into multiple lines.
    pub fn r#break(&self, value: impl Into<Document<'a>>) -> Document<'a> {
        r#break(self.allocate(value.into()))
    }

    /// Flattens a document.
    pub fn flatten(&self, value: impl Into<Document<'a>>) -> Document<'a> {
        flatten(self.allocate(value.into()))
    }

    /// Indents a document by a level.
    pub fn indent(&self, value: impl Into<Document<'a>>) -> Document<'a> {
        indent(self.allocate(value.into()))
    }

    /// Creates a document indented to a current column.
    pub fn offside(&self, value: impl Into<Document<'a>>, soft: bool) -> Document<'a> {
        offside(self.allocate(value.into()), soft)
    }

    /// Creates a sequence of documents.
    pub fn sequence(
        &self,
        values: impl IntoIterator<Item = impl Into<Document<'a>>>,
    ) -> Document<'a> {
        sequence(self.allocate_slice(values.into_iter().map(Into::into)))
    }

    /// Creates a concatenation of strings.
    pub fn strings<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> Document<'a> {
        self.allocate_str(values).into()
    }

    /// Creates a set of line suffixes.
    pub fn line_suffixes<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> Document<'a> {
        line_suffix(self.allocate_str(values))
    }

    /// Allocates a value.
    pub fn allocate<T>(&self, value: T) -> &'a T {
        Box::leak(Box::new_in(value, self.allocator.clone()))
    }

    /// Allocates a slice.
    pub fn allocate_slice<T>(&self, values: impl IntoIterator<Item = T>) -> &'a [T] {
        let mut vec = Vec::new_in(self.allocator.clone());

        vec.extend(values);

        Vec::leak(vec)
    }

    /// Allocates a string.
    pub fn allocate_str<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> &'a str {
        let mut vec = Vec::new_in(self.allocator.clone());

        for value in values {
            vec.extend(value.as_bytes().iter().copied());
        }

        str::from_utf8(Vec::leak(vec)).expect("utf-8 string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::offside;
    use alloc::alloc::Global;

    #[test]
    fn build_offside() {
        let builder = Builder::new(Global);

        assert_eq!(builder.offside("foo", false), offside(&"foo".into(), false));
    }
}
