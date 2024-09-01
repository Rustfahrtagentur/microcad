struct SrcRef {
    range: std::ops::Range<usize>,
    line_col: (u32, u32),
}

use crate::{parse::SourceFile, parser::Pair};

/*impl SrcRef {
    fn src(&self, source_file: &SourceFile) {

    }
}*/

impl From<Pair<'_>> for SrcRef {
    fn from(pair: Pair) -> Self {
        let (line, col) = pair.line_col();
        Self {
            range: pair.as_span().start()..pair.as_span().end(),
            line_col: (line as u32, col as u32),
        }
    }
}
