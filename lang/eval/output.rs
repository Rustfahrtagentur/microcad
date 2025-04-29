// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Trait which EvalContext is using to access or redirect the µcad code's console output.
pub trait Output {
    /// Print into output buffer.
    fn print(&mut self, what: String) -> std::io::Result<()>;
    /// Access captured output.
    fn output(&self) -> Option<String>;
}

/// Output what `__builtin::print` is printing to stdout.
pub struct Stdout;

impl Output for Stdout {
    /// Print into output buffer.
    fn print(&mut self, what: String) -> std::io::Result<()> {
        println!("{what}");
        Ok(())
    }
    fn output(&self) -> Option<String> {
        None
    }
}

/// Output buffer to catch what `__builtin::print` is printing.
pub struct Capture {
    buf: std::io::BufWriter<Vec<u8>>,
}

impl Capture {
    /// Create new capture buffer.
    pub fn new() -> Self {
        Self {
            buf: std::io::BufWriter::new(Vec::new()),
        }
    }
}

impl Default for Capture {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for Capture {
    fn print(&mut self, what: String) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.buf, "{what}")
    }
    fn output(&self) -> Option<String> {
        String::from_utf8(self.buf.buffer().into()).ok()
    }
}
