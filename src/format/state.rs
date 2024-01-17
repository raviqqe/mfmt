#[derive(Clone, Copy, Debug)]
pub struct State {
    indent: usize,
    broken: bool,
}

impl State {
    pub fn indent(&self) -> usize {
        self.indent
    }

    pub fn broken(&self) -> bool {
        self.broken
    }

    pub fn set_indent(self, indent: usize) -> Self {
        Self { indent, ..self }
    }

    pub fn set_broken(self, broken: bool) -> Self {
        Self { broken, ..self }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            indent: 0,
            broken: true,
        }
    }
}
