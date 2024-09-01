//! Remember source code position for diagnosis

use crate::{parse::SourceFile, src_ref::*};

enum Level {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Error => write!(f, "error"),
            Level::Warning => write!(f, "warning"),
            Level::Info => write!(f, "info"),
        }
    }
}

struct Diagnostic {
    pub src_ref: SrcRef,
    pub message: String,
    pub level: Level,
}

impl Diagnostic {
    pub fn new(src_ref: SrcRef, message: String, level: Level) -> Self {
        Self {
            src_ref,
            message,
            level,
        }
    }

    pub fn info(src: impl SrcReferrer, message: String) -> Self {
        Self::new(src.src_ref(), message, Level::Info)
    }

    pub fn warning(src: impl SrcReferrer, message: String) -> Self {
        Self::new(src.src_ref(), message, Level::Warning)
    }

    pub fn error(src: impl SrcReferrer, message: String) -> Self {
        Self::new(src.src_ref(), message, Level::Error)
    }

    pub fn pretty_print(&self, w: &mut dyn std::fmt::Write, source_file: &SourceFile) -> std::fmt::Result {
        writeln!(
            w,
            "{}: {}",
            self.level,
            self.message
        )?;
        writeln!(
            w,
            "  ---> {}:{}",
            source_file.file_name_as_str(),
            self.src_ref.at().unwrap(),
        )?;
        writeln!(
            w,
            "     |",
        )?;

        let line = source_file.get_line(self.src_ref.at().unwrap().line as usize - 1).unwrap_or("<no line>");

        writeln!(
            w,
            "{: >4} | {}",
            self.src_ref.at().unwrap().line,
            line
        )?;
        writeln!(
            w,
            "{: >4} | {}",
            "",
            " ".repeat(self.src_ref.at().unwrap().col as usize - 1) + &"^".repeat(self.src_ref.len().min(line.len())),
        )?;
        writeln!(
            w,
            "     |",
        )?;
        Ok(())
    }
}



/// Diagnostics for a single source file
struct SourceFileDiagnostics {
    source_file: std::rc::Rc<SourceFile>,
    diagnostics: Vec<Diagnostic>,
}

impl SourceFileDiagnostics {
    pub fn new(source_file: std::rc::Rc<SourceFile>) -> Self {
        Self {
            source_file,
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::info(src, message));
    }

    pub fn warning(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::warning(src, message));
    }

    pub fn error(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::error(src, message));
    }
}

impl std::fmt::Display for SourceFileDiagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for diagnostic in &self.diagnostics {
            diagnostic.pretty_print(f, &self.source_file)?;
        }
        Ok(())
    }
}

#[test]
fn test_diagnostics() {
    let source_file = std::rc::Rc::new(SourceFile::from_file(r#"../tests/std/algorithm_difference.µcad"#).unwrap());

    let mut diagnostics = SourceFileDiagnostics::new(source_file.clone());

    let mut body_iter = source_file.body.iter();
    diagnostics.info(body_iter.next().unwrap(), "This is an info".to_string());
    diagnostics.warning(body_iter.next().unwrap(), "This is a warning".to_string());
    diagnostics.error(body_iter.next().unwrap(), "This is an error".to_string());


    assert_eq!(diagnostics.diagnostics.len(), 3);
    eprintln!("{}", diagnostics);
}
