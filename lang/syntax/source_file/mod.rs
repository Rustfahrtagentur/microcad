// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::{src_ref::*, syntax::*};

/// µcad source file
#[derive(Clone, Default)]
pub struct SourceFile {
    /// Qualified name of the file if loaded from externals
    pub name: QualifiedName,
    /// Root code body.
    pub statements: StatementList,
    /// Name of loaded file.
    filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    pub source: String,

    /// Hash of the source file
    ///
    /// This hash is calculated from the source code itself
    /// This is used to map `SrcRef` -> `SourceFile`
    pub hash: u64,
}

impl SourceFile {
    /// Create new source file from existing source.
    pub fn new(statements: StatementList, source: String, hash: u64) -> Self {
        Self {
            statements,
            source,
            hash,
            ..Default::default()
        }
    }
    /// Return filename of loaded file or `<no file>`
    pub fn filename(&self) -> std::path::PathBuf {
        self.filename
            .clone()
            .unwrap_or(std::path::PathBuf::from(crate::invalid_no_ansi!(SOURCE)))
    }

    /// Return filename of loaded file or `<no file>`
    pub fn set_filename(&mut self, path: impl AsRef<std::path::Path>) {
        assert!(self.filename.is_none());
        self.filename = Some(path.as_ref().canonicalize().expect("path not found"))
    }

    /// Return filename of loaded file or `<no file>`
    pub fn filename_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|f| f.to_str().expect("File name error {filename:?}"))
            .unwrap_or(crate::invalid!(SOURCE))
    }

    /// Return the module name from the file name
    pub fn id(&self) -> Identifier {
        self.name.last().unwrap_or(&Identifier::none()).clone()
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.source.lines().nth(line)
    }

    /// return number of source code lines
    pub fn num_lines(&self) -> usize {
        self.source.lines().count()
    }

    /// Set file name.
    pub fn set_name(&mut self, name: QualifiedName) {
        self.name = name
    }
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements
            .iter()
            .try_for_each(|s| writeln!(f, "{s:?}"))
    }
}

impl TreeDisplay for SourceFile {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}SourceFile '{:?}' ({}):",
            "",
            self.id(),
            self.filename_as_str()
        )?;
        depth.indent();
        self.statements
            .iter()
            .try_for_each(|s| s.tree_print(f, depth))
    }
}

impl SrcReferrer for SourceFile {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        SrcRef::new(0..self.num_lines(), 0, 0, self.hash)
    }
}

#[test]
fn load_source_file_wrong_location() {
    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        log::info!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
