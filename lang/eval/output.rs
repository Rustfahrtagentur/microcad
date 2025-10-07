// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Trait which Context is using to access or redirect the µcad code's console output.
pub trait Output {
    /// Print into output buffer.
    fn print(&mut self, what: String) -> std::io::Result<()>;
    /// Access captured output.
    fn output(&self) -> Option<String>;
}

/// Output what `__builtin::print` is printing to stdout.
pub struct Stdout;

impl Stdout {
    /// Create new stdout output.
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

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
    pub fn new() -> Box<Self> {
        Box::new(Self {
            buf: std::io::BufWriter::new(Vec::new()),
        })
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
