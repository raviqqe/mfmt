use super::{Document, Str, r#break, flatten, indent, offside, sequence};
use allocator_api2::{alloc::Allocator, boxed::Box, vec::Vec};

/// Document builder.
#[derive(Clone, Debug)]
pub struct Builder<A: Allocator> {
    allocator: A,
}

impl<'a, A: Allocator + Clone> Builder<A> {
    /// Creates a document builder.
    pub fn new(allocator: A) -> Self {
        Self { allocator }
    }

    /// Returns an allocator.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Breaks a document into multiple lines.
    pub fn r#break(&self, value: impl Into<Document<'a, A>>) -> Document<'a, A> {
        r#break(self.allocate(value.into()))
    }

    /// Flattens a document.
    pub fn flatten(&self, value: impl Into<Document<'a, A>>) -> Document<'a, A> {
        flatten(self.allocate(value.into()))
    }

    /// Indents a document by a level.
    pub fn indent(&self, value: impl Into<Document<'a, A>>) -> Document<'a, A> {
        indent(self.allocate(value.into()))
    }

    /// Creates a document indented to a current column.
    pub fn offside(&self, value: impl Into<Document<'a, A>>, soft: bool) -> Document<'a, A> {
        offside(self.allocate(value.into()), soft)
    }

    /// Creates a sequence of documents.
    pub fn sequence(
        &self,
        values: impl IntoIterator<Item = impl Into<Document<'a, A>>>,
    ) -> Document<'a, A> {
        sequence(self.allocate_slice(values.into_iter().map(Into::into)))
    }

    /// Creates a concatenation of strings.
    pub fn strings<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> Document<'a, A> {
        Document::String(Str::Owned(self.allocate_str(values)))
    }

    /// Creates a set of line suffixes.
    pub fn line_suffixes<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> Document<'a, A> {
        Document::LineSuffix(Str::Owned(self.allocate_str(values)))
    }

    /// Allocates a value.
    pub fn allocate<T>(&self, value: T) -> Box<T, A> {
        Box::new_in(value, self.allocator.clone())
    }

    /// Allocates a slice.
    pub fn allocate_slice<T>(&self, values: impl IntoIterator<Item = T>) -> Box<[T], A> {
        let mut vec = Vec::new_in(self.allocator.clone());

        vec.extend(values);

        vec.into_boxed_slice()
    }

    /// Allocates a string.
    pub fn allocate_str<'b>(&self, values: impl IntoIterator<Item = &'b str>) -> Box<str, A> {
        let mut vec = Vec::new_in(self.allocator.clone());

        for value in values {
            vec.extend(value.as_bytes().iter().copied());
        }

        let (pointer, allocator) = Box::into_raw_with_allocator(vec.into_boxed_slice());

        // SAFETY: A concatenation of strings is a valid UTF-8 string.
        unsafe { Box::from_raw_in(pointer as *mut str, allocator) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{line_suffix, offside};
    use alloc::rc::Rc;
    use allocator_api2::alloc::{AllocError, Global, Layout};
    use core::{cell::Cell, ptr::NonNull};

    #[derive(Clone, Default)]
    struct CountingAllocator {
        count: Rc<Cell<usize>>,
    }

    unsafe impl Allocator for CountingAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            self.count.set(self.count.get() + 1);

            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, pointer: NonNull<u8>, layout: Layout) {
            self.count.set(self.count.get() - 1);

            unsafe { Global.deallocate(pointer, layout) }
        }
    }

    #[test]
    fn build_offside() {
        let builder = Builder::new(Global);

        assert_eq!(
            builder.offside("foo", false),
            offside(Box::new("foo".into()), false)
        );
    }

    #[test]
    fn build_strings() {
        let builder = Builder::new(Global);

        assert_eq!(builder.strings(["foo", "bar"]), "foobar".into());
    }

    #[test]
    fn build_line_suffixes() {
        let builder = Builder::new(Global);

        assert_eq!(builder.line_suffixes(["foo", "bar"]), line_suffix("foobar"));
    }

    #[test]
    fn deallocate_documents() {
        let allocator = CountingAllocator::default();
        let builder = Builder::new(allocator.clone());
        let document = builder.sequence([
            builder.indent(builder.strings(["foo", "bar"])),
            builder.line_suffixes(["baz"]),
        ]);
        let clone = document.clone();

        assert_ne!(allocator.count.get(), 0);

        drop(document);
        drop(clone);

        assert_eq!(allocator.count.get(), 0);
    }
}
