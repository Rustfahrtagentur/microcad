use std::path::PathBuf;

use microcad_render::svg::SvgRenderer;

use crate::*;

pub struct SvgExporter {
    filename: PathBuf,
}

impl Exporter for SvgExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(settings.filename().unwrap()),
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["svg"]
    }

    fn export(&mut self, node: microcad_render::Node) -> microcad_core::Result<()> {
        let file = std::fs::File::create(&self.filename)?;

        use microcad_render::Renderer2D;
        let mut renderer = SvgRenderer::default();
        renderer.set_output(Box::new(file))?;
        renderer.render_node(node)
    }
}