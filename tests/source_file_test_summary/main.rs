use std::io::Write;

fn generate_header(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "# Test Summary")?;
    writeln!(w, "")?;
    Ok(())
}

fn generate_table_header(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "| Test file | SVG | Tree dump | Result |")?;
    writeln!(w, "|-----------|-----|-----------|--------|")?;
    Ok(())
}

fn generate_table_row(w: &mut impl Write, test_case: &str) -> std::io::Result<()> {
    let log_file = format!("../output/{test_case}.log");
    let svg_file = format!("../output/{test_case}.µcad.svg");
    let tree_file = format!("../output/{test_case}.tree.dump");

    // Open the log file and check if it contains the string "error"
    // If it does, then we should mark the test as failed
    let result = if log_file.contains("error:") {
        ":x:"
    } else {
        ":white_check_mark:"
    };

    writeln!(w, "| [{test_case}.µcad](../test_cases/{test_case}.µcad) | <img src=\"{svg_file}\" alt=\"{test_case}\" width=\"100\"/> | [Tree]({tree_file}) | [{result}]({log_file}) |")?;
    Ok(())
}

/// scan folder
fn scan(path: &std::path::Path, extension: &str) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut test_cases = Vec::new();

    // read given directory
    for entry in std::fs::read_dir(path)?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            let file_name = entry.file_name().into_string().unwrap();
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
            .unwrap()
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

    let mut w = std::io::BufWriter::new(std::fs::File::create(
        "tests/source_file_test_summary/README.md",
    )?);
    generate_table(&mut w, &test_cases)?;
    w.flush()?;

    Ok(())
}
