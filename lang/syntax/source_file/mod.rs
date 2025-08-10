// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};

/// µcad source file
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceFile {
    /// Qualified name of the file if loaded from externals
    pub name: QualifiedName,
    /// Root code body.
    pub statements: StatementList,
    /// Name of loaded file.
    pub filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    pub source: String,

    /// Hash of the source file
    ///
    /// This hash is calculated from the the source code itself
    /// This is used to map `SrcRef` -> `SourceFile`
    pub hash: u64,
}

impl SourceFile {
    /// Return filename of loaded file or `<no file>`
    pub fn filename(&self) -> std::path::PathBuf {
        self.filename
            .clone()
            .unwrap_or(std::path::PathBuf::from(crate::invalid_no_ansi!(SOURCE)))
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
        match &self.filename {
            Some(filename) => Identifier(Refer::new(
                filename
                    .file_stem()
                    .expect("cannot get file stem")
                    .to_str()
                    .expect("File name error {filename:?}")
                    .into(),
                SrcRef::new(0..0, 0, 0, self.hash),
            )),
            None => Identifier::none(),
        }
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

    /// Resolve into SymbolNode
    pub fn resolve(&self, parent: Option<Symbol>) -> Symbol {
        Rc::new(self.clone()).resolve_rc(parent)
    }

    /// Like resolve but with `Rc<SourceFile>`
    pub fn resolve_rc(self: Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let name = self.filename_as_str();
        log::debug!("Resolving source file {name}");
        let symbol = Symbol::new(SymbolDefinition::SourceFile(self.clone()), parent);
        symbol.borrow_mut().children = self.statements.fetch_symbol_map(Some(symbol.clone()));
        log::trace!("Resolved source file {name}:\n{symbol}");
        symbol
    }
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl TreeDisplay for SourceFile {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeIndent) -> std::fmt::Result {
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

/// print syntax via std::fmt::Display
pub struct FormatSyntax<'a, T: TreeDisplay>(pub &'a T);

impl<T: TreeDisplay> std::fmt::Display for FormatSyntax<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(f, 2.into())
    }
}

#[test]
fn load_source_file() {
    let source_file = SourceFile::load(r#"../tests/test_cases/ops/difference.µcad"#);
    if let Err(ref err) = source_file {
        log::error!("{err}");
    }

    let source_file = source_file.expect("test error");

    let first_statement = source_file.statements.first().expect("test error");
    match first_statement {
        Statement::Use(u) => {
            use crate::src_ref::SrcReferrer;
            assert_eq!(
                u.src_ref().source_slice(&source_file.source),
                "use __builtin::*;"
            );
        }
        _ => panic!(),
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
