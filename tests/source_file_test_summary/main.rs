// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::Write;

/// Generate the header of the test summary
fn generate_header(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "# Test Summary")?;
    writeln!(w)?;
    Ok(())
}

/// Generate the header of the table
fn generate_table_header(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "| Test file | SVG | Tree dump | Result |")?;
    writeln!(w, "|-----------|-----|-----------|--------|")?;
    Ok(())
}

/// Compare two files and return true if they are equal
fn file_equal(file1: impl AsRef<std::path::Path>, file2: impl AsRef<std::path::Path>) -> bool {
    std::fs::read_to_string(file1)
        .expect("error reading file2")
        .replace("\r\n", "\n")
        .trim()
        == std::fs::read_to_string(file2)
            .expect("error reading file2")
            .replace("\r\n", "\n")
            .trim()
}

/// Generate a row of the table
fn generate_table_row(w: &mut impl Write, test_case: &str) -> std::io::Result<()> {
    let log_file = format!("tests/output/{test_case}.log");
    let svg_file = format!("tests/output/{test_case}.µcad.svg");
    let result_tree_file = format!("tests/output/{test_case}.tree.dump");
    let reference_tree_file = format!("tests/test_cases/{test_case}.tree.dump");

    println!(
        "Checking test case: {} {}",
        reference_tree_file,
        std::path::Path::new(&reference_tree_file).exists()
    );

    // Print current directory
    println!("Current directory: {:?}", std::env::current_dir()?);

    // Get absolute path of the reference tree file
    println!("Reference tree file: {:?}", reference_tree_file);

    let test_case_column = format!("[{test_case}.µcad](tests/test_cases/{test_case}.µcad)",);

    let svg_column = if std::path::Path::new(&svg_file).exists() {
        format!("<img src=\"{svg_file}\" alt=\"{test_case}\" width=\"100\"/>")
    } else {
        String::from(":heavy_exclamation_mark: No SVG output")
    };

    let tree_column = if std::path::Path::new(&reference_tree_file).exists() {
        // Check if result and reference tree files are the same
        let result = if !file_equal(&result_tree_file, &reference_tree_file) {
            ":x:"
        } else {
            ":white_check_mark:"
        };

        format!("{result} [Result]({result_tree_file}) [Reference]({reference_tree_file})")
    } else {
        format!(":heavy_exclamation_mark: [Result]({result_tree_file}) No reference")
    };

    // Open the log file and check if it contains the string "error"
    // If it does, then we should mark the test as failed
    let result_column = format!(
        "[{result}]({log_file})",
        result = if log_file.contains("error:") {
            ":x:"
        } else {
            ":white_check_mark:"
        },
    );

    writeln!(
        w,
        "| {test_case_column} | {svg_column} | {tree_column} | {result_column} |"
    )?;
    Ok(())
}

/// scan folder
fn scan(path: &std::path::Path, extension: &str) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut test_cases = Vec::new();

    // read given directory
    for entry in std::fs::read_dir(path)?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            let file_name = entry
                .file_name()
                .into_string()
                .expect("Failed to convert file name");
            // check if directory or file
            if file_type.is_dir() {
                test_cases.append(&mut scan(&entry.path(), extension)?);
            } else if file_type.is_file() && file_name.ends_with(&format!(".{extension}")) {
                test_cases.push(entry.path());
            }
        }
    }

    Ok(test_cases)
}

fn generate_table(w: &mut impl Write, test_cases: &[std::path::PathBuf]) -> std::io::Result<()> {
    generate_header(w)?;
    generate_table_header(w)?;
    for test_case in test_cases {
        let test_case_stripped = test_case
            .strip_prefix("tests/test_cases")
            .expect("Failed to strip prefix")
            .to_string_lossy()
            .to_string()
            .replace("\\", "/")
            .replace(".µcad", "");

        generate_table_row(w, &test_case_stripped)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let test_cases = scan(std::path::Path::new("tests/test_cases"), "µcad")?;

    let mut w = std::io::BufWriter::new(std::fs::File::create("TEST_SUMMARY.md")?);
    generate_table(&mut w, &test_cases)?;
    w.flush()?;

    Ok(())
}
