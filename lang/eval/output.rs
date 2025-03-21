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
    pub fn get(&mut self) -> Result<String, std::string::FromUtf8Error> {
        let bytes = self.buf.buffer();
        String::from_utf8(bytes.into())
    }

    /// Print into output buffer
    pub fn print(&mut self, what: String) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(self.buf, "{what}")
    }
}
