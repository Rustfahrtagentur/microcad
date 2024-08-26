use crate::Vec3;
use manifold_rs::Mesh;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle<T>(pub T, pub T, pub T);

impl Triangle<Vertex> {
    pub fn normal(&self) -> Vec3 {
        (self.2.pos - self.0.pos).cross(self.1.pos - self.0.pos)
    }
}

#[derive(Default, Clone)]
pub struct TriangleMesh {
    vertices: Vec<Vertex>,
    triangle_indices: Vec<Triangle<u32>>,
}

impl TriangleMesh {
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangle_indices.clear();
    }

    pub fn fetch_triangles(&self) -> Vec<Triangle<Vertex>> {
        let mut triangles = Vec::with_capacity(self.triangle_indices.len());
        for t in &self.triangle_indices {
            triangles.push(Triangle(
                self.vertices[t.0 as usize],
                self.vertices[t.1 as usize],
                self.vertices[t.2 as usize],
            ));
        }
        triangles
    }

    pub fn append(&mut self, other: &TriangleMesh) {
        let offset = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&other.vertices);
        for t in &other.triangle_indices {
            self.triangle_indices
                .push(Triangle(t.0 + offset, t.1 + offset, t.2 + offset));
        }
    }
}

impl From<Mesh> for TriangleMesh {
    fn from(mesh: Mesh) -> Self {
        let vertices = mesh.vertices();
        let indices = mesh.indices();

        let mut triangle_mesh = TriangleMesh {
            vertices: Vec::with_capacity(vertices.len() / 6),
            triangle_indices: Vec::with_capacity(indices.len() / 3),
        };

        // TODO: We could use unsafe std::ptr::copy and cast::transmute to avoid deep copy
        // of vertices and indices
        for i in (0..vertices.len()).step_by(6) {
            let vertex = Vertex {
                pos: Vec3::new(
                    vertices[i] as f64,
                    vertices[i + 1] as f64,
                    vertices[i + 2] as f64,
                ),
                normal: Vec3::new(
                    vertices[i + 3] as f64,
                    vertices[i + 4] as f64,
                    vertices[i + 5] as f64,
                ),
            };
            triangle_mesh.vertices.push(vertex);
        }

        for i in (0..indices.len()).step_by(3) {
            let triangle = Triangle(indices[i], indices[i + 1], indices[i + 2]);
            triangle_mesh.triangle_indices.push(triangle);
        }

        triangle_mesh
    }
}
