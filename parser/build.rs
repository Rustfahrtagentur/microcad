use rustmod::*;

fn main() {
    if let Err(err) = scan_folder("..") {
        panic!("ERROR: {err}");
    }
}
