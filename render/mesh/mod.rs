use crate::*;
use microcad_core::{export, geo3d, Error, Scalar};

mod ply;
mod stl;
struct MeshRenderer {
    precision: Scalar,
    triangle_mesh: geo3d::TriangleMesh,
}

impl Renderer for MeshRenderer {
    fn precision(&self) -> Scalar {
        self.precision
    }
}

impl Default for MeshRenderer {
    fn default() -> Self {
        Self {
            precision: 0.1,
            triangle_mesh: geo3d::TriangleMesh::default(),
        }
    }
}

impl Renderer3D for MeshRenderer {
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> microcad_core::Result<()> {
        self.triangle_mesh.append(mesh);
        Ok(())
    }

    fn render_node(&mut self, node: Node) -> microcad_core::Result<()> {
        let inner = node.borrow();

        match &*inner {
            NodeInner::Export(_) | NodeInner::Group | NodeInner::Root => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
                return Ok(());
            }
            NodeInner::Algorithm(algorithm) => {
                let new_node = algorithm.process_3d(self, node.clone())?;
                self.render_node(new_node)?;
            }
            NodeInner::Geometry3D(geometry) => {
                self.render_geometry(geometry)?;
            }
            NodeInner::Renderable3D(renderable) => {
                let geometry = renderable.request_geometry(self)?;
                self.render_geometry(&geometry)?;
            }
            NodeInner::Transform(_) => unimplemented!(),
            NodeInner::Geometry2D(_) | NodeInner::Renderable2D(_) => {
                return Err(Error::NotImplemented);
            }
        }

        Ok(())
    }
}

impl MeshRenderer {
    fn export_stl(&self, settings: &export::ExportSettings) -> microcad_core::Result<()> {
        assert!(settings.filename().is_some());
        let file = std::fs::File::create(settings.filename().unwrap())?;
        let mut file = std::io::BufWriter::new(file);
        let mut stl = stl::StlWriter::new(&mut file);

        let triangles = self.triangle_mesh.fetch_triangles();
        for triangle in triangles {
            stl.write_triangle(&triangle)?;
        }

        Ok(())
    }

    fn export_ply(&self, settings: &export::ExportSettings) -> microcad_core::Result<()> {
        assert!(settings.filename().is_some());
        let file = std::fs::File::create(settings.filename().unwrap())?;
        let mut file = std::io::BufWriter::new(file);
        let mut ply_writer = ply::PlyWriter::new(&mut file)?;

        let mesh = &self.triangle_mesh;
        ply_writer.header_element_vertex3d(mesh.vertices().len())?;
        ply_writer.header_element_face(mesh.triangle_indices().len())?;
        ply_writer.header_end()?;

        ply_writer.vertices(mesh.vertices())?;
        ply_writer.tri_faces(mesh.triangle_indices())?;

        Ok(())
    }
}

impl microcad_core::Exporter for MeshRenderer {
    fn from_settings(_: &export::ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            precision: 0.1,
            triangle_mesh: geo3d::TriangleMesh::default(),
        })
    }

    fn export(&mut self, _: Node) -> microcad_core::Result<()> {
        todo!("Implement export for MeshRenderer")
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["stl", "ply"]
    }
}
