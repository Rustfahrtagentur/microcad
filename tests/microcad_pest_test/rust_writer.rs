// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Test code writer

/// Rust test code writer
pub struct RustWriter<'a> {
    w: &'a mut dyn std::io::Write,
    indent: usize,
}

impl<'a> RustWriter<'a> {
    /// Create new writer
    pub fn new(w: &'a mut dyn std::io::Write) -> Self {
        Self { w, indent: 0 }
    }

    /// Open a Rust scope
    pub fn begin_scope(&mut self, s: &str) -> Result<(), std::io::Error> {
        if s.is_empty() {
            self.writeln("{")?;
        } else {
            self.writeln(format!("{s} {{").as_str())?;
        }

        self.indent += 1;
        Ok(())
    }

    /// Write code line
    pub fn writeln(&mut self, s: &str) -> Result<(), std::io::Error> {
        write!(self.w, "{}", "    ".repeat(self.indent))?;
        writeln!(self.w, "{}", s)?;
        Ok(())
    }

    /// Write code
    pub fn write(&mut self, s: &str) -> Result<(), std::io::Error> {
        self.writeln(s)?;
        Ok(())
    }

    /// End a Rust scope
    pub fn end_scope(&mut self) -> Result<(), std::io::Error> {
        self.writeln("}")?;
        self.indent -= 1;
        Ok(())
    }
}
