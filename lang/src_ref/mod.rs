//! Source code reference

mod line_col;
mod refer;

pub use line_col::*;
pub use refer::*;

use crate::parser::Pair;

pub trait SrcReferrer {
    fn src_ref(&self) -> SrcRef;
}

/// We want to be able to use SrcRef directly in functions with `impl SrcReferrer` argument
impl SrcReferrer for SrcRef {
    fn src_ref(&self) -> SrcRef {
        self.clone()
    }
}

/// We want to be able to use type references as well
impl<T: SrcReferrer> SrcReferrer for &T {
    fn src_ref(&self) -> SrcRef {
        (*self).src_ref()
    }
}

/// Reference into a source file
///
/// *Hint*: Source file is not part of `SrcRef` and must be provided from outside
#[derive(Clone, Debug, Default)]
pub struct SrcRef(pub Option<SrcRefInner>);

impl SrcRef {
    /// Create new `SrcRef?
    /// - `range`: Position in file
    /// - `line`: Line number (0..) in file
    /// - `col`: Column number (ß..) in file
    pub fn new(range: std::ops::Range<usize>, line: u32, col: u32) -> Self {
        Self(Some(SrcRefInner {
            range,
            at: LineCol { line, col },
        }))
    }

    /// return length of
    pub fn len(&self) -> usize {
        self.0.as_ref().map(|s| s.range.len()).unwrap_or(0)
    }

    /// return true if code base is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::ops::Deref for SrcRef {
    type Target = Option<SrcRefInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A reference in the source code
#[derive(Clone, Debug, Default)]
pub struct SrcRefInner {
    /// Range in bytes
    pub range: std::ops::Range<usize>,
    /// Line and column (aka position)
    pub at: LineCol,
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
        Some(self.cmp(other))
    }
}

impl Eq for SrcRef {}

impl Ord for SrcRef {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl SrcRef {
    /// return slice to code base
    pub fn source_slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.0.as_ref().unwrap().range.to_owned()]
    }

    /// merge two `SrcRef` into a single one by
    pub fn merge(lhs: SrcRef, rhs: SrcRef) -> SrcRef {
        match (lhs, rhs) {
            (SrcRef(Some(lhs)), SrcRef(Some(rhs))) => SrcRef(Some(SrcRefInner {
                range: {
                    // paranoia check
                    assert!(lhs.range.end <= rhs.range.end);
                    assert!(lhs.range.start <= rhs.range.start);

                    lhs.range.start..rhs.range.end
                },
                at: lhs.at,
            })),
            (SrcRef(Some(hs)), SrcRef(None)) | (SrcRef(None), SrcRef(Some(hs))) => SrcRef(Some(hs)),
            _ => unreachable!(),
        }
    }

    /// Return a `Src` from from `Vec`, by looking at first at and last element only.
    /// Assume that position of SrcRefs in v is sorted
    pub fn from_vec<T: SrcReferrer>(v: &[T]) -> SrcRef {
        match v.is_empty() {
            true => SrcRef(None),
            false => Self::merge(v.first().unwrap().src_ref(), v.last().unwrap().src_ref()),
        }
    }

    /// Return line (0..) and column (0..) in source code or `None` if not available
    pub fn at(&self) -> Option<LineCol> {
        self.0.as_ref().map(|s| s.at.clone())
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
