/// Line and column within a source code file
#[derive(Clone, Debug, Default)]
pub struct LineCol {
    /// Line number (0..)
    pub line: u32,
    /// Column number (0..)
    pub col: u32,
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
