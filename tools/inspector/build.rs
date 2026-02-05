//! Build script for microcad inspector

fn main() {
    slint_build::compile("ui/mainwindow.slint").expect("No error");
}
