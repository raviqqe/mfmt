use core::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub struct FormatOptions {
    indent: NonZeroUsize,
    space: char,
}

impl FormatOptions {
    pub const fn new(indent: NonZeroUsize) -> Self {
        Self { indent, space: ' ' }
    }

    pub const fn indent(&self) -> NonZeroUsize {
        self.indent
    }

    pub const fn space(self) -> char {
        self.space
    }

    pub const fn set_indent(self, indent: NonZeroUsize) -> Self {
        Self { indent, ..self }
    }

    pub const fn set_space(self, space: char) -> Self {
        Self { space, ..self }
    }
}
