// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Output buffer to catch what __builtin::print is printing
pub struct Output {
    buf: std::io::BufWriter<Vec<u8>>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            buf: std::io::BufWriter::new(Vec::new()),
        }
    }
}

impl Output {
    /// return output buffer as String
    pub fn get(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.buf.buffer().into())
    }

    /// Print into output buffer
    pub fn print(&mut self, what: String) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.buf, "{what}")
    }
}
