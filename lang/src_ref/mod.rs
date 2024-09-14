// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code reference

mod line_col;
mod refer;

pub use line_col::*;
pub use refer::*;

use crate::parser::*;

/// Elements holding a source code reference shall implement this trait
pub trait SrcReferrer {
    /// return source code reference
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
    /// Create new `SrcRef`
    /// - `range`: Position in file
    /// - `line`: Line number within file
    /// - `col`: Column number within file
    pub fn new(range: std::ops::Range<usize>, line: u32, col: u32, source_file_hash: u64) -> Self {
        Self(Some(SrcRefInner {
            range,
            at: LineCol { line, col },
            source_file_hash,
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

    /// return source file hash
    /// - `0` if not `SrcRefInner` is none
    /// - `u64` if `SrcRefInner` is some
    ///
    /// This is used to map `SrcRef` -> `SourceFile`
    pub fn source_hash(&self) -> u64 {
        self.0.as_ref().map(|s| s.source_file_hash).unwrap_or(0)
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
    /// Line and column
    pub at: LineCol,
    /// Hash of the source code file to map `SrcRef` -> `SourceFile`
    pub source_file_hash: u64,
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
    pub fn merge(lhs: impl SrcReferrer, rhs: impl SrcReferrer) -> SrcRef {
        match (lhs.src_ref(), rhs.src_ref()) {
            (SrcRef(Some(lhs)), SrcRef(Some(rhs))) => {
                let hash = {
                    if lhs.source_file_hash != rhs.source_file_hash {
                        0
                    } else {
                        lhs.source_file_hash
                    }
                };

                SrcRef(Some(SrcRefInner {
                    range: {
                        // paranoia check
                        assert!(lhs.range.end <= rhs.range.end);
                        assert!(lhs.range.start <= rhs.range.start);

                        lhs.range.start..rhs.range.end
                    },
                    at: lhs.at,
                    source_file_hash: hash,
                }))
            }
            (SrcRef(Some(hs)), SrcRef(None)) | (SrcRef(None), SrcRef(Some(hs))) => SrcRef(Some(hs)),
            _ => SrcRef(None),
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
            pair.source_hash(),
        )
    }
}

#[test]
fn test_src_ref() {
    let input = "geo3d::cube(size_x = 3.0, size_y = 3.0, size_z = 3.0);";

    let cube = 7..11;
    let size_y = 26..32;

    let cube = SrcRef::new(cube, 1, 0, 0);
    let size_y = SrcRef::new(size_y, 1, 0, 0);

    assert_eq!(cube.source_slice(input), "cube");
    assert_eq!(size_y.source_slice(input), "size_y");
}
