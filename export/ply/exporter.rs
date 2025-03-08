use super::*;
use crate::Exporter;
use microcad_core::*;
use microcad_lang::*;
use std::path::PathBuf;

/// PLY exporter
pub struct PlyExporter {
    filename: PathBuf,
    precision: Scalar,
}

impl Exporter for PlyExporter {
    fn from_settings(settings: &microcad_core::ExportSettings) -> microcad_core::CoreResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            filename: settings.file_path()?,
            precision: settings.render_precision()?,
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["ply"]
    }

    fn export(&mut self, node: objects::ObjectNode) -> Result<(), microcad_core::CoreError> {
        use microcad_core::geo3d::*;

        let mut renderer = MeshRenderer::new(self.precision);
        let node = objects::bake3d(&mut renderer, node)?;
        renderer.render_node(node)?;

        let file = std::fs::File::create(&self.filename)?;
        let mut file = std::io::BufWriter::new(file);
        let mut ply_writer = PlyWriter::new(&mut file)?;

        let mesh = renderer.triangle_mesh;
        ply_writer.header_element_vertex3d(mesh.vertices.len())?;
        ply_writer.header_element_face(mesh.triangle_indices.len())?;
        ply_writer.header_end()?;

        ply_writer.vertices(&mesh.vertices)?;
        ply_writer.tri_faces(&mesh.triangle_indices)?;

        Ok(())
    }
}
