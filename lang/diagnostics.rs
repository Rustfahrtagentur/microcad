//! Remember source code position for diagnosis

use core::fmt;

/// Position in source code file
pub struct SourceLocation {
    /// Path of the source it was parsed from
    path: std::path::PathBuf,
    /// Line number in the source
    line: usize,
    /// Column in the source
    column: usize,
}

impl SourceLocation {
    pub fn from_pair(path: impl AsRef<std::path::Path>, pair: crate::parser::Pair) -> Self {
        use std::path::*;

        let (line, column) = pair.as_span().start_pos().line_col();
        Self {
            path: PathBuf::from(path.as_ref()),
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
