// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Remember source code position for diagnosis

use crate::{parse::GetSourceFileByHash, src_ref::*};

/// The level of the diagnostic
#[derive(Debug, Clone)]
pub enum Level {
    Trace,
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "trace"),
            Level::Error => write!(f, "error"),
            Level::Warning => write!(f, "warning"),
            Level::Info => write!(f, "info"),
        }
    }
}

/// A trait to add diagnostics with different levels conveniently
pub trait PushDiagnostic {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic);

    fn trace(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diagnostic(Diagnostic::Trace(src.src_ref(), message));
    }
    fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diagnostic(Diagnostic::Info(src.src_ref(), message));
    }
    fn warning(&mut self, src: impl SrcReferrer, error: anyhow::Error) -> anyhow::Result<()> {
        self.push_diagnostic(Diagnostic::Warning(src.src_ref(), error));
        Ok(())
    }
    fn error(&mut self, src: impl SrcReferrer, error: anyhow::Error) -> anyhow::Result<()> {
        self.push_diagnostic(Diagnostic::Error(src.src_ref(), error));
        Ok(())
    }
}

#[derive(Debug)]
pub enum Diagnostic {
    Trace(SrcRef, String),
    Info(SrcRef, String),
    Error(SrcRef, anyhow::Error),
    Warning(SrcRef, anyhow::Error),
}

impl Diagnostic {
    pub fn level(&self) -> Level {
        match self {
            Diagnostic::Trace(_, _) => Level::Trace,
            Diagnostic::Info(_, _) => Level::Info,
            Diagnostic::Error(_, _) => Level::Error,
            Diagnostic::Warning(_, _) => Level::Warning,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Diagnostic::Trace(_, message) => message.to_string(),
            Diagnostic::Info(_, message) => message.to_string(),
            Diagnostic::Error(_, error) => error.to_string(),
            Diagnostic::Warning(_, error) => error.to_string(),
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
        let source_file = source_file_by_hash
            .get_source_file_by_src_ref(&src_ref)
            .unwrap();

        match &src_ref {
            SrcRef(None) => writeln!(w, "{}: {}", self.level(), self.message())?,
            SrcRef(Some(ref src_ref)) => {
                writeln!(w, "{}: {}", self.level(), self.message())?;
                writeln!(w, "  ---> {}:{}", source_file.filename(), src_ref.at)?;
                writeln!(w, "     |",)?;

                let line = source_file
                    .get_line(src_ref.at.line as usize - 1)
                    .unwrap_or("<no line>");

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

impl SrcReferrer for Diagnostic {
    fn src_ref(&self) -> SrcRef {
        match self {
            Diagnostic::Trace(src, _) => src.clone(),
            Diagnostic::Info(src, _) => src.clone(),
            Diagnostic::Error(src, _) => src.clone(),
            Diagnostic::Warning(src, _) => src.clone(),
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diagnostic::Trace(src, message) => write!(f, "trace: {}: {}", src, message),
            Diagnostic::Info(src, message) => write!(f, "info: {}: {}", src, message),
            Diagnostic::Error(src, error) => write!(f, "error: {}: {}", src, error),
            Diagnostic::Warning(src, error) => write!(f, "warning: {}: {}", src, error),
        }
    }
}

#[derive(Debug, Default)]
pub struct Diagnostics {
    /// We have a vec of source file diagnostics because we want to keep track of diagnostics for each source file separately
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_file_by_hash: &impl GetSourceFileByHash,
    ) -> std::io::Result<()> {
        for source_file_diagnostics in &self.diagnostics {
            source_file_diagnostics.pretty_print(w, source_file_by_hash)?;
        }
        Ok(())
    }
}

impl PushDiagnostic for Diagnostics {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

#[test]
fn test_diagnostics() {
    let source_file =
        crate::parse::SourceFile::load(r#"../tests/std/algorithm_difference.µcad"#).unwrap();

    let mut diagnostics = Diagnostics::default();

    let mut body_iter = source_file.body.iter();
    use anyhow::anyhow;

    diagnostics.info(body_iter.next().unwrap(), "This is an info".to_string());
    diagnostics.warning(body_iter.next().unwrap(), anyhow!("This is a warning"));

    diagnostics.error(body_iter.next().unwrap(), anyhow!("This is an error"));

    assert_eq!(diagnostics.diagnostics.len(), 3);
    diagnostics
        .pretty_print(
            &mut std::io::stdout(),
            source_file
                .get_source_file_by_hash(source_file.hash())
                .unwrap(),
        )
        .unwrap();
}
