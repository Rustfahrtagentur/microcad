use cmake::Config;

fn main() {
    // Environment variable CMAKE_PREFIX_PATH is used to find the manifold library
    // On Windows, set up rustup as follows: to use the GNU toolchain
    // `$ rustup default stable-x86_64-pc-windows-gnu`

    // Uncomment the following line to set the CMAKE_PREFIX_PATH environment variable
    //std::env::set_var("CMAKE_PREFIX_PATH", "C:/msys64/mingw64/bin");
    std::env::set_var("CMAKE_GENERATOR", "Ninja");

    let dst = Config::new("../manifold").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-=manifold");
}
