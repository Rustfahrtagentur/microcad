//! Remember source code position for diagnosis

use crate::{parse::SourceFile, src_ref::*};


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
pub trait DiagnosticAdd {
    fn add(&mut self, diagnostic: Diagnostic);

    fn trace(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::new(src.src_ref(), message, Level::Trace));
    }
    fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::new(src.src_ref(), message, Level::Info));
    }
    fn warning(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::new(src.src_ref(), message, Level::Warning));
    }
    fn error(&mut self, src: impl SrcReferrer, message: String) {
        self.add(Diagnostic::new(src.src_ref(), message, Level::Error));
    }
}



/// A diagnostic containing a source reference, a message and a level
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub src_ref: SrcRef,
    pub message: String,
    pub level: Level,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(src_ref: SrcRef, message: String, level: Level) -> Self {
        Self {
            src_ref,
            message,
            level,
        }
    }

    pub fn pretty_print(
        &self,
        w: &mut dyn std::fmt::Write,
        source_file: &SourceFile,
    ) -> std::fmt::Result {
        writeln!(w, "{}: {}", self.level, self.message)?;
        writeln!(
            w,
            "  ---> {}:{}",
            source_file.filename(),
            self.src_ref.at().unwrap(),
        )?;
        writeln!(w, "     |",)?;

        let line = source_file
            .get_line(self.src_ref.at().unwrap().line as usize - 1)
            .unwrap_or("<no line>");

        writeln!(w, "{: >4} | {}", self.src_ref.at().unwrap().line, line)?;
        writeln!(
            w,
            "{: >4} | {}",
            "",
            " ".repeat(self.src_ref.at().unwrap().col as usize - 1)
                + &"^".repeat(self.src_ref.len().min(line.len())),
        )?;
        writeln!(w, "     |",)?;
        Ok(())
    }
}

/// Diagnostics for a single source file
#[derive(Debug)]
pub struct SourceFileDiagnostics {
    /// The source is an `Rc` because we want to share the source file between the diagnostics and the context
    /// This way we can keep track of the source file and the diagnostics separately.
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

    pub fn pretty_print(
        &self,
        w: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        for diagnostic in &self.diagnostics {
            diagnostic.pretty_print(w, self.source_file.as_ref())?;
        }
        Ok(())
    }
}

impl DiagnosticAdd for SourceFileDiagnostics {
    fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

impl DiagnosticAdd for &mut SourceFileDiagnostics {
    fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
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


#[derive(Debug, Default)]
pub struct Diagnostics {
    /// We have a vec of source file diagnostics because we want to keep track of diagnostics for each source file separately
    diagnostics: Vec<SourceFileDiagnostics>,

    /// Trace with indices to the diagnostics vector
    trace: Vec<usize>,
}

impl Diagnostics {
    pub fn new(source_file: std::rc::Rc<SourceFile>) -> Self {
        Self {
            diagnostics: vec![SourceFileDiagnostics::new(source_file.clone())],
            trace: Vec::new(),
        }
    }

    pub fn current_source_file(&self) -> std::rc::Rc<SourceFile> {
        self.diagnostics.last().map(|d| d.source_file.clone()).unwrap()
    }

    pub fn push(&mut self, source_file: std::rc::Rc<SourceFile>) {
        self.trace.push(self.diagnostics.len());
        self.diagnostics.push(SourceFileDiagnostics::new(source_file));
    }

    pub fn pop(&mut self) {
        self.trace.pop();
    }

    pub fn pretty_print(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        for source_file_diagnostics in &self.diagnostics {
            source_file_diagnostics.pretty_print(w)?;
        }
        Ok(())
    }

    pub fn print_backtrace(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        for index in &self.trace {
            self.diagnostics[*index].pretty_print(w)?;
        }
        Ok(())
    }
}

impl DiagnosticAdd for Diagnostics {
    fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.last_mut().unwrap().add(diagnostic);
    }
}


#[test]
fn test_diagnostics() {
    let source_file = std::rc::Rc::new(
        SourceFile::load(r#"../tests/std/algorithm_difference.µcad"#).unwrap(),
    );

    let mut diagnostics = SourceFileDiagnostics::new(source_file.clone());

    let mut body_iter = source_file.body.iter();
    diagnostics.info(body_iter.next().unwrap(), "This is an info".to_string());
    diagnostics.warning(body_iter.next().unwrap(), "This is a warning".to_string());
    diagnostics.error(body_iter.next().unwrap(), "This is an error".to_string());

    assert_eq!(diagnostics.diagnostics.len(), 3);
    eprintln!("{}", diagnostics);
}
