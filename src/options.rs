/// Format options.
#[derive(Clone, Copy, Debug)]
pub struct FormatOptions {
    indent: usize,
    space: char,
}

impl FormatOptions {
    /// Creates options for indentation by spaces.
    pub const fn new(indent: usize) -> Self {
        Self { indent, space: ' ' }
    }

    /// Creates options for indentation by tabs.
    pub fn tab() -> Self {
        Self {
            indent: 1,
            space: '\t',
        }
    }

    /// Returns an indent size.
    pub const fn indent(&self) -> usize {
        self.indent
    }

    /// Returns a space character.
    pub const fn space(self) -> char {
        self.space
    }

    /// Sets an indent size.
    pub const fn set_indent(self, indent: usize) -> Self {
        Self { indent, ..self }
    }

    /// Sets a space character.
    pub const fn set_space(self, space: char) -> Self {
        Self { space, ..self }
    }
}
