// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::{eval::*, src_ref::*, syntax::*, value::*};

/// Trait to get a source file by its hash
pub trait GetSourceFileByHash {
    /// Get a source file by its hash
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile>;

    /// Convenience function to get a source file by from a `SrcRef`
    fn get_source_file_by_src_ref(&self, src_ref: &impl SrcReferrer) -> Option<&SourceFile> {
        self.get_source_file_by_hash(src_ref.src_ref().source_hash())
    }

    /// Convenience function to get source slice by `SrcRef`
    fn get_source_string(&self, src_ref: &impl SrcReferrer) -> Option<&str> {
        if let Some(source_file) = self.get_source_file_by_src_ref(&src_ref) {
            Some(src_ref.src_ref().source_slice(&source_file.source))
        } else {
            None
        }
    }

    /// return a string describing the given source code position
    fn ref_str(&self, src_ref: &impl SrcReferrer) -> String {
        format!(
            "{}:{}",
            self.get_source_file_by_src_ref(src_ref)
                .expect("Source file not found")
                .filename_as_str(),
            src_ref.src_ref(),
        )
    }
}

/// µcad source file
#[derive(Clone, Debug, Default)]
pub struct SourceFile {
    /// Root code body
    pub body: Vec<Statement>,
    /// Name of loaded file or `None`
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
    /// Printed instead of a file name if file name could not be retrieved
    pub const NO_FILE: &str = "<no file>";

    /// Return filename of loaded file or `<no file>`
    pub fn filename_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|path| path.to_str().unwrap_or(Self::NO_FILE))
            .unwrap_or(Self::NO_FILE)
    }

    /// Return the namespace name from the file name
    pub fn namespace_name_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|path| {
                path.file_stem()
                    .expect("cannot get file stem")
                    .to_str()
                    .unwrap_or(Self::NO_FILE)
            })
            .unwrap_or(Self::NO_FILE)
    }

    /// Return source file hash
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.source.lines().nth(line)
    }
}

impl Eval for SourceFile {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        Body::evaluate_vec(&self.body, context)
    }
}

/// We can get a source file by its hash
impl GetSourceFileByHash for SourceFile {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        if self.hash == hash {
            Some(self)
        } else {
            None
        }
    }
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.body.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl PrintSyntax for SourceFile {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}SourceFile '{}':", "", self.filename_as_str())?;
        self.body
            .iter()
            .try_for_each(|s| s.print_syntax(f, depth + 1))
    }
}

/// print syntax via std::fmt::Display
pub struct FormatSyntax<'a, T: PrintSyntax>(pub &'a T);

impl<T: PrintSyntax> std::fmt::Display for FormatSyntax<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_syntax(f, 0)
    }
}

#[test]
fn load_source_file() {
    use log::*;

    crate::env_logger_init();

    let source_file = SourceFile::load(r#"../tests/test_cases/algorithm/difference.µcad"#);
    if let Err(ref err) = source_file {
        error!("{err}");
    }

    let source_file = source_file.expect("test error");

    let first_statement = source_file.body.first().expect("test error");
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
    use log::*;

    crate::env_logger_init();

    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        info!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
