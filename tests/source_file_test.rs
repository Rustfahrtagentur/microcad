// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Struct with source file test, given as a filename
///
/// A test with a filename ending with `_fail.µcad` is supposed to fail.
#[cfg(test)]
pub(crate) struct SourceFileTest {
    filename: std::path::PathBuf,
}

#[cfg(test)]
impl SourceFileTest {
    /// Create a new source file test.
    pub fn new<T: AsRef<std::path::Path>>(filename: T) -> Self {
        Self {
            filename: std::path::PathBuf::from(filename.as_ref()),
        }
    }

    /// This function is called for every `.µcad` file in `./test_cases`.
    ///
    /// It will evaluate a `.µcad` file and generate:
    /// * `.symbol_tree`: A dump of the root symbol tree.
    /// * `.model_tree`: A dump of the model tree.
    /// * `.log`: An error log file.
    /// * (TODO) `.svg`: An output SVG file, if the test produces a 2D geometry.
    /// * (TODO) `.stl`: An output STL file, if the test produces a 3D geometry.
    ///
    /// All output files will be stored in `./.test`.
    ///
    /// You can place `.log`, `.model_tree` and other reference files next to the respective `.µcad` files.
    pub fn test(&self) {
        use microcad_lang::diag::Diag;

        let mut context =
            crate::context_for_file(self.filename.to_str().expect("Valid filename str"));

        let symbol_tree = context.root();
        self.write_and_compare(symbol_tree, "symbol_tree");

        match context.eval() {
            Ok(model) => {
                use microcad_core::*;
                use microcad_lang::model::{ExportCommand as Export, OutputType};
                use std::rc::Rc;

                self.write_and_compare(model.clone(), "model_tree");

                if !context.has_errors() {
                    // If we have a 2D model, export model to SVG
                    match model.final_output_type() {
                        OutputType::Geometry2D => {
                            Export {
                                filename: self.output_filename("svg").into(),
                                resolution: RenderResolution::default(),
                                exporter: Rc::new(microcad_export::svg::SvgExporter),
                            }
                            .export(&model)
                            .expect("No error");
                        }
                        OutputType::Geometry3D => {
                            Export {
                                filename: self.output_filename("stl").into(),
                                resolution: RenderResolution::coarse(),
                                exporter: Rc::new(microcad_export::stl::StlExporter),
                            }
                            .export(&model)
                            .expect("No error");
                        }
                        OutputType::NotDetermined => {}
                        _ => panic!("Invalid geometry output"),
                    }
                }
            }
            Err(err) => {
                panic!("Error evaluating context: {err}");
            }
        }

        self.write_and_compare(context.diagnosis(), "log");

        match (context.has_errors(), self.is_failing()) {
            (true, false) => panic!("Error evaluating"),
            (true, true) => log::trace!("This test is supposed to fail."),
            (false, true) => panic!("This test is supposed to fail but no errors occured"),
            (false, false) => log::trace!("This test is ok"),
        }
    }

    /// A test is failing if the filename ends with `_fail.µcad`
    fn is_failing(&self) -> bool {
        self.filename
            .to_str()
            .expect("Valid path")
            .ends_with("_fail.µcad")
    }

    /// Write the content of a `Display` to a file.
    fn write_and_compare(&self, value: impl std::fmt::Display, extension: &str) {
        // Write value to file
        {
            let output_filename = self.output_filename(extension);
            let file = std::fs::File::create(&output_filename)
                .unwrap_or_else(|_| panic!("Failed to create file: {output_filename}"));
            use std::io::Write;
            let mut writer = std::io::BufWriter::new(file);
            write!(writer, "{value}").expect("Valid file write")
        }

        // Compare
        self.compare_output_to_reference_file(extension);
    }

    /// Generate a reference filename from `filename` with `extension`.
    ///
    /// `syntax/multiplicity.µcad` -> `syntax/multiplicity.log`
    fn reference_filename(&self, extension: &str) -> String {
        let mut path = std::path::PathBuf::from("../tests/test_cases").join(self.filename.clone());
        path.set_extension(extension);
        path.to_string_lossy().into_owned()
    }

    /// Generate an output filename from `filename` with `extension`.
    ///
    /// `syntax/multiplicity.µcad` -> `./test/syntax_multiplicity.log`
    fn output_filename(&self, extension: &str) -> String {
        let mut filename = self.filename.clone();
        filename.set_extension(extension);

        // Create directory tree for the test in case it does not exist yet
        let output_filename = format!("../tests/.test/{}", filename.display());
        let path = std::path::Path::new(&output_filename);
        std::fs::create_dir_all(path.parent().expect("Filename with parent"))
            .expect("Create directory");

        output_filename
    }

    /// Compares the output to reference file.
    fn compare_output_to_reference_file(&self, extension: &str) {
        let output = &self.output_filename(extension);
        let reference = &self.reference_filename(extension);
        log::trace!("Compare \"{output}\" with \"{reference}\"");

        use std::path::Path;
        match (Path::new(output).exists(), Path::new(reference).exists()) {
            (true, true) => Self::compare_files(output, reference),
            (true, false) => log::info!(r#"No reference file "{reference}""#),
            (false, true) => {
                log::error!(
                    r#"There is reference file "{reference}", but no output file "{output}"!"#
                )
            }
            (false, false) => unreachable!("This should not happen."),
        }
    }

    /// Compare the content of two files.
    #[cfg(test)]
    fn compare_files(file1: &str, file2: &str) {
        let content1 = std::fs::read_to_string(file1)
            .unwrap_or_else(|_| panic!("Failed to read file: {file1}"));
        let content2 = std::fs::read_to_string(file2)
            .unwrap_or_else(|_| panic!("Failed to read file: {file2}"));
        if content1 != content2 {
            log::error!("\nFile {file1}:\n{content1}\nFile {file2}:\n{content2}");
            panic!("File contents differ: {file1} vs {file2}");
        }
    }
}
