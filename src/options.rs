use core::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub struct FormatOptions {
    pub(crate) indent: usize,
    pub(crate) space: char,
}

impl FormatOptions {
    pub const fn new(indent: NonZeroUsize) -> Self {
        Self {
            indent: indent.into(),
            space: ' ',
        }
    }

    pub const fn indent(self, space: char) -> Self {
        Self { space, ..self }
    }

    pub const fn space(self, space: char) -> Self {
        Self { space, ..self }
    }
}
