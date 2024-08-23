use cmake::Config;

fn main() {
    // Environment variable CMAKE_PREFIX_PATH is used to find the manifold library
    // On Windows, set up rustup as follows: to use the GNU toolchain
    // `$ rustup default stable-x86_64-pc-windows-gnu`

    // Uncomment the following line to set the CMAKE_PREFIX_PATH environment variable
    //std::env::set_var("CMAKE_PREFIX_PATH", "C:/msys64/mingw64/bin");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::env::set_var("CMAKE_PREFIX_PATH", format!("{out_dir}/build/glm"));

    std::env::set_var("CMAKE_GENERATOR", "Ninja");
    std::env::set_var("CMAKE_BUILD_TYPE", "Release");

    let glm = Config::new("../glm").cxxflag("/EHsc").build();
    println!("cargo:rustc-link-search=native={}", glm.display());

    let manifold = Config::new("../manifold")
        .cxxflag("/EHsc")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!("cargo:rustc-link-search={out_dir}/lib");
    //    println!("cargo:rustc-link-search={out_dir}/build/src/polygon");
    println!("cargo:rustc-link-search=native={}", manifold.display());

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
