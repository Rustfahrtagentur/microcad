#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("manifold_rs.h");

        type Manifold;

        fn sphere(radius: f64) -> UniquePtr<Manifold>;
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;

        type Mesh;

        fn mesh_from_manifold(manifold: &Manifold) -> UniquePtr<Mesh>;
        fn vertices(self: &Mesh) -> UniquePtr<CxxVector<f32>>;
        fn indices(self: &Mesh) -> UniquePtr<CxxVector<u32>>;
    }
}

pub struct Manifold(cxx::UniquePtr<ffi::Manifold>);

impl Manifold {
    pub fn sphere(radius: f64) -> Self {
        let manifold = ffi::sphere(radius);
        Self(manifold)
    }

    pub fn cube(x_size: f64, y_size: f64, z_size: f64) -> Self {
        let manifold = ffi::cube(x_size, y_size, z_size);
        Self(manifold)
    }

    pub fn mesh(&self) -> Mesh {
        let mesh = ffi::mesh_from_manifold(&self.0);
        Mesh(mesh)
    }
}

pub struct Mesh(cxx::UniquePtr<ffi::Mesh>);

impl Mesh {
    pub fn vertices(&self) -> Vec<f32> {
        let vertices_binding = self.0.vertices();
        let vertices = vertices_binding.as_ref().unwrap().as_slice();
        vertices.to_vec()
    }

    pub fn indices(&self) -> Vec<u32> {
        let indices_binding = self.0.indices();
        let indices = indices_binding.as_ref().unwrap().as_slice();
        indices.to_vec()
    }
}

#[test]
fn test_manifold() {
    let sphere = ffi::sphere(1.0);

    let mesh = ffi::mesh_from_manifold(&sphere);

    let vertices_binding = mesh.vertices();
    let vertices = vertices_binding.as_ref().unwrap().as_slice();
    assert!(!vertices.is_empty());

    let indices_binding = mesh.indices();
    let indices = indices_binding.as_ref().unwrap().as_slice();
    assert!(!indices.is_empty());
}
