use core::fmt;

pub struct SourceLocation {
    /// Path of the source it was parsed from
    path: std::path::PathBuf,
    /// Line number in the source
    line: usize,
    /// Column in the source
    column: usize,
}

impl SourceLocation {
    pub fn from_pair<'i>(
        path: impl AsRef<std::path::Path>,
        pair: pest::iterators::Pair<'i, crate::Rule>,
    ) -> Self {
        let (line, column) = pair.as_span().start_pos().line_col();
        Self {
            path: std::path::PathBuf::from(path.as_ref()),
            line,
            column,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}:{}", self.path, self.line, self.column)
    }
}
