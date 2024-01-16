use core::num::NonZeroUsize;

/// Format options.
#[derive(Clone, Copy, Debug)]
pub struct FormatOptions {
    indent: NonZeroUsize,
    space: char,
}

impl FormatOptions {
    /// Creates options.
    pub const fn new(indent: NonZeroUsize) -> Self {
        Self { indent, space: ' ' }
    }

    /// Creates options for indentation by tabs.
    pub fn tab() -> Self {
        Self {
            indent: NonZeroUsize::new(1).unwrap(),
            space: '\t',
        }
    }

    /// Returns an indent size.
    pub const fn indent(&self) -> NonZeroUsize {
        self.indent
    }

    /// Returns a space character.
    pub const fn space(self) -> char {
        self.space
    }

    /// Sets an indent size.
    pub const fn set_indent(self, indent: NonZeroUsize) -> Self {
        Self { indent, ..self }
    }

    /// Sets a space character.
    pub const fn set_space(self, space: char) -> Self {
        Self { space, ..self }
    }
}
