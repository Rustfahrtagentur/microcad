// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handling of diagnostics with source code references

mod diag_handler;
mod diag_list;
mod level;

pub use diag_handler::*;
pub use diag_list::*;
pub use level::*;

use crate::{parse::*, src_ref::*};

/// A trait to add diagnostics with different levels conveniently
pub trait PushDiag {
    /// Push a diagnostic message (must be implemented)
    fn push_diag(&mut self, diag: Diag) -> crate::eval::Result<()>;

    /// Push new trace message
    fn trace(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diag::Trace(Refer::new(message, src.src_ref())))
            .unwrap();
    }
    /// Push new informative message
    fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diag::Info(Refer::new(message, src.src_ref())))
            .unwrap();
    }
    /// Push new warning
    fn warning(&mut self, src: impl SrcReferrer, error: anyhow::Error) -> crate::eval::Result<()> {
        self.push_diag(Diag::Warning(Refer::new(error, src.src_ref())))
    }
    /// Push new error
    fn error(&mut self, src: impl SrcReferrer, error: anyhow::Error) -> crate::eval::Result<()> {
        self.push_diag(Diag::Error(Refer::new(error, src.src_ref())))
    }
}

/// Diagnostic message with source code reference
#[derive(Debug)]
pub enum Diag {
    /// Trace message with source code reference attached
    Trace(Refer<String>),
    /// Informative message with source code reference attached
    Info(Refer<String>),
    /// Warning with source code reference attached
    Warning(Refer<anyhow::Error>),
    /// Error  with source code reference attached
    Error(Refer<anyhow::Error>),
}

impl Diag {
    /// Get diagnostic level
    pub fn level(&self) -> Level {
        match self {
            Diag::Trace(_) => Level::Trace,
            Diag::Info(_) => Level::Info,
            Diag::Warning(_) => Level::Warning,
            Diag::Error(_) => Level::Error,
        }
    }

    /// get message (errors will be serialized)
    pub fn message(&self) -> String {
        match self {
            Diag::Trace(message) => message.to_string(),
            Diag::Info(message) => message.to_string(),
            Diag::Warning(error) => error.to_string(),
            Diag::Error(error) => error.to_string(),
        }
    }

    /// Pretty print the diagnostic
    ///
    /// This will print the diagnostic to the given writer, including the source code reference
    ///
    /// # Arguments
    ///
    /// * `w` - The writer to write to
    /// * `source_file_by_hash` - Hash provider to get the source file by hash
    ///
    /// This will print:
    ///
    /// ```text
    /// error: This is an error
    ///   ---> filename:1:8
    ///     |
    ///  1  | module circle(radius: length) {}
    ///     |        ^^^^^^
    /// ```
    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_file_by_hash: &impl GetSourceFileByHash,
    ) -> std::io::Result<()> {
        let src_ref = self.src_ref();
        let source_file = source_file_by_hash.get_source_file_by_src_ref(&src_ref);

        match &src_ref {
            SrcRef(None) => writeln!(w, "{}: {}", self.level(), self.message())?,
            SrcRef(Some(ref src_ref)) => {
                writeln!(w, "{}: {}", self.level(), self.message())?;
                writeln!(
                    w,
                    "  ---> {}:{}",
                    if let Some(source_file) = source_file {
                        source_file.filename_as_str()
                    } else {
                        "<no file>"
                    },
                    src_ref.at
                )?;
                writeln!(w, "     |",)?;

                let line = if let Some(source_file) = source_file {
                    source_file
                        .get_line(src_ref.at.line as usize - 1)
                        .unwrap_or("<no line>")
                } else {
                    "<no file>"
                };
                writeln!(w, "{: >4} | {}", src_ref.at.line, line)?;

                writeln!(
                    w,
                    "{: >4} | {}",
                    "",
                    " ".repeat(src_ref.at.col as usize - 1)
                        + &"^".repeat(src_ref.range.len().min(line.len())),
                )?;
                writeln!(w, "     |",)?;
            }
        }

        Ok(())
    }
}

impl SrcReferrer for Diag {
    fn src_ref(&self) -> SrcRef {
        match self {
            Diag::Trace(message) => message.src_ref(),
            Diag::Info(message) => message.src_ref(),
            Diag::Warning(error) => error.src_ref(),
            Diag::Error(error) => error.src_ref(),
        }
    }
}

impl std::fmt::Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diag::Trace(message) => write!(f, "trace: {}: {message}", self.src_ref()),
            Diag::Info(message) => write!(f, "info: {}: {message}", self.src_ref()),
            Diag::Warning(error) => write!(f, "warning: {}: {error}", self.src_ref()),
            Diag::Error(error) => write!(f, "error: {}: {error}", self.src_ref()),
        }
    }
}

#[test]
fn test_diag_list() {
    let source_file =
        crate::parse::SourceFile::load(r#"../tests/test_cases/algorithm_difference.µcad"#).unwrap();

    let mut diagnostics = DiagList::default();

    let mut body_iter = source_file.body.iter();
    use anyhow::anyhow;

    diagnostics.info(body_iter.next().unwrap(), "This is an info".to_string());
    diagnostics
        .warning(body_iter.next().unwrap(), anyhow!("This is a warning"))
        .unwrap();
    diagnostics
        .error(body_iter.next().unwrap(), anyhow!("This is an error"))
        .unwrap();

    assert_eq!(diagnostics.len(), 3);
    let mut output = std::io::Cursor::new(Vec::new());
    diagnostics
        .pretty_print(
            &mut output,
            source_file
                .get_source_file_by_hash(source_file.hash())
                .unwrap(),
        )
        .unwrap();

    // Hol den Inhalt des Puffers
    let result = String::from_utf8(output.into_inner()).expect("Invalid UTF-8");
    assert_eq!(
        result,
        "info: This is an info
  ---> ../tests/test_cases/algorithm_difference.µcad:1:1
     |
   1 | use * from std;
     | ^^^^^^^^^^^^^^^
     |
warning: This is a warning
  ---> ../tests/test_cases/algorithm_difference.µcad:4:1
     |
   4 | export(\"{OUTPUT_FILE}.stl\") algorithm::difference() {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: This is an error
  ---> ../tests/test_cases/algorithm_difference.µcad:10:1
     |
  10 | export(\"{OUTPUT_FILE}.svg\") algorithm::difference() {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
"
    );
}
