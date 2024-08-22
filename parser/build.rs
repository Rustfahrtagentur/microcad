use rustmod::*;

fn main() {
    if let Err(err) = scan_project_files("..") {
        panic!("ERROR: {err}");
    }
}
