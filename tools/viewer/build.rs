fn main() {
    // Compile all Slint files in the project
    slint_build::compile("ui/MainWindow.slint").unwrap();
}
