// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code references.
//!
//! All errors which occur when parsing or evaluating µcad code need to address a point within the code where they appeared.
//! To do so, a bunch of structs provide this functionality:
//!
//! - [`SrcRef`] boxes [`SrcRefInner`] which includes all necessary reference information like *line*/*column* and a
//!   hash to identify the source file.
//! - [`Refer`] encapsulates any syntax element and puts a [`SrcRef`] beside it.
//! - [`SrcReferrer`] is a trait which provides unified access to the [`SrcRef`] (e.g. implemented by [`Refer`].

mod line_col;
mod refer;
mod src_referrer;

pub use line_col::*;
pub use refer::*;
pub use src_referrer::*;

use crate::parser::*;

/// Reference into a source file
///
/// *Hint*: Source file is not part of `SrcRef` and must be provided from outside
#[derive(Clone, Debug, Default)]
pub struct SrcRef(pub Option<Box<SrcRefInner>>);

impl SrcRef {
    /// Create new `SrcRef`
    /// - `range`: Position in file
    /// - `line`: Line number within file
    /// - `col`: Column number within file
    pub fn new(
        range: std::ops::Range<usize>,
        line: usize,
        col: usize,
        source_file_hash: u64,
    ) -> Self {
        Self(Some(Box::new(SrcRefInner {
            range,
            at: LineCol { line, col },
            source_file_hash,
        })))
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

impl std::ops::Deref for SrcRef {
    type Target = Option<Box<SrcRefInner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    /// return length of `SrcRef`
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

    /// return slice to code base
    pub fn source_slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.0.as_ref().expect("SrcRef").range.to_owned()]
    }

    /// merge two `SrcRef` into a single one by
    pub fn merge(lhs: impl SrcReferrer, rhs: impl SrcReferrer) -> SrcRef {
        match (lhs.src_ref(), rhs.src_ref()) {
            (SrcRef(Some(lhs)), SrcRef(Some(rhs))) => {
                let source_file_hash = lhs.source_file_hash;

                // TODO Not sure if this is correct.
                // Can we actually merge two ranges of SrcRef?
                if lhs.range.end > rhs.range.start || lhs.range.start > rhs.range.end {
                    return SrcRef(Some(lhs));
                }

                SrcRef(Some(Box::new(SrcRefInner {
                    range: {
                        // paranoia check
                        assert!(lhs.range.end <= rhs.range.end);
                        assert!(lhs.range.start <= rhs.range.start);

                        lhs.range.start..rhs.range.end
                    },
                    at: lhs.at,
                    source_file_hash,
                })))
            }
            (SrcRef(Some(hs)), SrcRef(None)) | (SrcRef(None), SrcRef(Some(hs))) => SrcRef(Some(hs)),
            _ => SrcRef(None),
        }
    }

    /// Return a `Src` from from `Vec`, by looking at first at and last element only.
    /// Assume that position of SrcRefs in v is sorted
    pub fn from_vec<T: SrcReferrer>(v: &[T]) -> SrcRef {
        match (v.first(), v.last()) {
            (None, None) => SrcRef(None),
            (Some(first), Some(last)) => Self::merge(first.src_ref(), last.src_ref()),
            _ => unreachable!(),
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
            line,
            col,
            pair.source_hash(),
        )
    }
}

#[test]
fn test_src_ref() {
    crate::env_logger_init();

    let input = "geo3d::cube(size_x = 3.0, size_y = 3.0, size_z = 3.0);";

    let cube = 7..11;
    let size_y = 26..32;

    let cube = SrcRef::new(cube, 1, 0, 0);
    let size_y = SrcRef::new(size_y, 1, 0, 0);

    assert_eq!(cube.source_slice(input), "cube");
    assert_eq!(size_y.source_slice(input), "size_y");
}
