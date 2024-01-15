#[derive(Clone, Copy, Debug, Default)]
pub struct FormatOptions {
    pub(crate) indent: usize,
    pub(crate) space: char,
}

impl FormatOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn indent(self, space: char) -> Self {
        Self { space, ..self }
    }

    pub fn space(self, space: char) -> Self {
        Self { space, ..self }
    }
}
