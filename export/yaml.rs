use std::{fs::File, path::PathBuf};

use microcad_render::Node;

use crate::*;

pub struct YamlExporter {
    filename: PathBuf,
}

impl Exporter for YamlExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(settings.filename().unwrap()),
        })
    }

    fn export(&mut self, node: Node) -> microcad_core::Result<()> {
        let file = File::create(&self.filename)?;
        let mut writer = std::io::BufWriter::new(&file);

        use std::io::Write;

        use microcad_core::render::tree::Depth;

        for child in node.descendants() {
            for _ in 0..child.depth() {
                write!(writer, "  ")?;
            }
            writeln!(writer, "- {:?}", child.borrow())?;
        }

        Ok(())
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["yaml"]
    }
}
