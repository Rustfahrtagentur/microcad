use std::{hash::Hasher, ops::Deref};

use crate::parser::Pair;

#[derive(Clone, Debug, Default)]
pub struct LineCol {
    line: u32,
    col: u32,
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Clone, Debug, Default)]
pub struct SrcRef(pub Option<SrcRefInner>);

impl SrcRef {
    fn new(range: std::ops::Range<usize>, line: u32, col: u32) -> Self {
        Self(Some(SrcRefInner {
            range,
            at: LineCol { line, col },
        }))
    }
}

impl Deref for SrcRef {
    type Target = Option<SrcRefInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A reference in the source code
#[derive(Clone, Debug, Default)]
pub struct SrcRefInner {
    /// Range in bytes
    range: std::ops::Range<usize>,
    /// Line and column (aka position)
    at: LineCol,
}

impl std::fmt::Display for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => write!(f, "{}", s.at),
            _ => write!(f, "<no_ref>"),
        }
    }
}

impl PartialEq for SrcRef {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for SrcRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Eq for SrcRef {}

impl Ord for SrcRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl SrcRef {
    fn source_slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.0.as_ref().unwrap().range.to_owned()]
    }

    pub fn merge(lhs: SrcRef, rhs: SrcRef) -> SrcRef {
        match (lhs, rhs) {
            (SrcRef(Some(lhs)), SrcRef(Some(rhs))) => SrcRef(Some(SrcRefInner {
                range: lhs.range.start..rhs.range.end,
                at: lhs.at,
            })),
            _ => unreachable!(),
        }
    }

    /// Return a Src from from Vec, by looking at first at and last element only.
    /// Assume that position of SrcRefs in v is sorted
    pub fn from_vec<T: SrcReferer>(v: &Vec<T>) -> SrcRef {
        match v.is_empty() {
            true => SrcRef(None),
            false => Self::merge(v.first().unwrap().src_ref(), v.last().unwrap().src_ref()),
        }
    }
}

impl From<Pair<'_>> for SrcRef {
    fn from(pair: Pair) -> Self {
        let (line, col) = pair.line_col();
        Self::new(
            pair.as_span().start()..pair.as_span().end(),
            line as u32,
            col as u32,
        )
    }
}

pub trait SrcReferer {
    fn src_ref(&self) -> SrcRef;
}

#[test]
fn test_src_ref() {
    let input = "geo3d::cube(size_x = 3.0, size_y = 3.0, size_z = 3.0);";

    let cube = 7..11;
    let size_y = 26..32;

    let cube = SrcRef::new(cube, 1, 0);
    let size_y = SrcRef::new(size_y, 1, 0);

    assert_eq!(cube.source_slice(input), "cube");
    assert_eq!(size_y.source_slice(input), "size_y");
}
