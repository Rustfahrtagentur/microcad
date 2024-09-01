use crate::{parse::SourceFile, parser::Pair};

#[derive(Clone, Debug)]
struct LineCol {
    line: u32,
    col: u32,
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// A reference in the source code
#[derive(Debug)]
struct SrcRef {
    /// Range in bytes
    range: std::ops::Range<usize>,
    /// Line and column (aka position)
    at: LineCol,
}

impl std::fmt::Display for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.at)
    }
}

impl SrcRef {
    fn src(&self, source_file: &SourceFile) {}

    fn source_slice<'a>(&self, src: &'a str) -> (&'a str, LineCol) {
        (&src[self.range.to_owned()], self.at.to_owned())
    }
}

impl From<Pair<'_>> for SrcRef {
    fn from(pair: Pair) -> Self {
        let (line, col) = pair.line_col();
        Self {
            range: pair.as_span().start()..pair.as_span().end(),
            at: LineCol {
                line: line as u32,
                col: col as u32,
            },
        }
    }
}
