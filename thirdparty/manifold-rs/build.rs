use cmake::Config;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::env::set_var("CMAKE_PREFIX_PATH", format!("{out_dir}/build/glm"));
    std::env::set_var("CMAKE_GENERATOR", "Ninja");
    std::env::set_var("CMAKE_BUILD_TYPE", "Release");

    let glm = Config::new("../glm").cxxflag("/EHsc").build();
    println!("cargo:rustc-link-search=native={}", glm.display());

    Config::new("../manifold")
        .cxxflag("/EHsc")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!("cargo:rustc-link-search={out_dir}/lib");

    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .file("src/manifold_rs.cpp")
        .include("./src")
        .include("../manifold/src/manifold/include")
        .include("../manifold/src/utilities/include")
        .include(format!("{out_dir}/build/_deps/glm-src"))
        .include(format!("{out_dir}/include"))
        .define("MANIFOLD_RS_LIBRARY", "1")
        .compile("manifold_rs");

    println!("cargo:rustc-link-lib=static=manifold");
    println!("cargo:rustc-link-lib=static=polygon");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/manifold_rs.h");
    println!("cargo:rerun-if-changed=src/manifold_rs.cpp");
}
