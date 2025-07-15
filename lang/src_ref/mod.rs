// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code references.
//!
//! All errors which occur while *parsing* or *evaluating* µcad code need to be reported and
//! therefore need to address a place in the code where they did appear.
//! A bunch of structs from this module provide this functionality:
//!
//! - [`SrcRef`] boxes a [`SrcRefInner`] which itself includes all necessary reference
//!   information like *line*/*column* and a hash to identify the source file.
//! - [`Refer`] encapsulates any syntax element and puts a [`SrcRef`] beside it.
//! - [`SrcReferrer`] is a trait which provides unified access to the [`SrcRef`]
//!   (e.g. implemented by [`Refer`]).

mod line_col;
mod refer;
mod src_referrer;

pub use line_col::*;
pub use refer::*;
pub use src_referrer::*;

use crate::parser::*;
use derive_more::Deref;

/// Reference into a source file.
///
/// *Hint*: Source file is not part of `SrcRef` and must be provided from outside
#[derive(Clone, Default, Deref)]
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
/// A reference into the source code
#[derive(Clone, Default)]
pub struct SrcRefInner {
    /// Range in bytes
    pub range: std::ops::Range<usize>,
    /// Line and column
    pub at: LineCol,
    /// Hash of the source code file to map `SrcRef` -> `SourceFile`
    pub source_file_hash: u64,
}

impl std::fmt::Display for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Some(s) => write!(f, "{}", s.at),
            _ => write!(f, "<no_ref>"),
        }
    }
}

impl std::fmt::Debug for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => write!(
                f,
                "{} ({}..{}) in {:#x}",
                s.at, s.range.start, s.range.end, s.source_file_hash
            ),
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

    /// Return slice to code base.
    pub fn source_slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.0.as_ref().expect("SrcRef").range.to_owned()]
    }

    /// Merge two `SrcRef` into a single one.
    ///
    /// `SrcRef(None)` is returned if:
    /// - ranges not in correct order (warning in log),
    /// - references are not in the same file (warning in log),
    /// - or `lhs` and `rhs` are both `None`.
    pub fn merge(lhs: &impl SrcReferrer, rhs: &impl SrcReferrer) -> SrcRef {
        match (lhs.src_ref(), rhs.src_ref()) {
            (SrcRef(Some(lhs)), SrcRef(Some(rhs))) => {
                if lhs.source_file_hash == rhs.source_file_hash {
                    let source_file_hash = lhs.source_file_hash;

                    if lhs.range.end > rhs.range.start || lhs.range.start > rhs.range.end {
                        log::warn!("ranges not in correct order");
                        SrcRef(None)
                    } else {
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
                } else {
                    log::warn!("references are not in the same file");
                    SrcRef(None)
                }
            }
            (SrcRef(Some(hs)), SrcRef(None)) | (SrcRef(None), SrcRef(Some(hs))) => SrcRef(Some(hs)),
            _ => SrcRef(None),
        }
    }

    /// Merge all given source references to one
    ///
    /// All  given source references must have the same hash otherwise panics!
    pub fn merge_all<S: SrcReferrer>(referrers: impl Iterator<Item = S>) -> SrcRef {
        let mut result = SrcRef(None);
        for referrer in referrers {
            if let Some(src_ref) = referrer.src_ref().0 {
                if let SrcRef(Some(result)) = &mut result {
                    if result.source_file_hash != src_ref.source_file_hash {
                        panic!("can only merge source references of the same file");
                    }
                    if src_ref.range.start < result.range.start {
                        result.range.start = src_ref.range.start;
                        result.at = src_ref.at;
                    }
                    result.range.end = std::cmp::max(src_ref.range.end, result.range.end);
                } else {
                    result = SrcRef(Some(src_ref));
                }
            }
        }
        result
    }

    /// Return line and column in source code or `None` if not available.
    pub fn at(&self) -> Option<LineCol> {
        self.0.as_ref().map(|s| s.at.clone())
    }
}

#[test]
fn merge_all() {
    use std::ops::Range;
    assert_eq!(
        SrcRef::merge_all(
            [
                SrcRef::new(Range { start: 5, end: 8 }, 1, 6, 123),
                SrcRef::new(Range { start: 8, end: 10 }, 2, 1, 123),
                SrcRef::new(Range { start: 12, end: 16 }, 3, 1, 123),
                SrcRef::new(Range { start: 0, end: 10 }, 1, 1, 123),
            ]
            .iter(),
        ),
        SrcRef::new(Range { start: 0, end: 16 }, 1, 1, 123),
    );
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
    let input = "geo3d::cube(size_x = 3.0, size_y = 3.0, size_z = 3.0);";

    let cube = 7..11;
    let size_y = 26..32;

    let cube = SrcRef::new(cube, 1, 0, 0);
    let size_y = SrcRef::new(size_y, 1, 0, 0);

    assert_eq!(cube.source_slice(input), "cube");
    assert_eq!(size_y.source_slice(input), "size_y");
}
