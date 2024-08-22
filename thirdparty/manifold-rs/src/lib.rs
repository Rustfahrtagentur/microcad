#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("manifold_rs.h");

        type Manifold;

        fn sphere(radius: f64) -> UniquePtr<Manifold>;
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;
    }
}

#[test]
fn test_manifold() {
    let sphere = ffi::sphere(1.0);
    let cube = ffi::cube(1.0, 2.0, 3.0);
}
