// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests for each µcad files in the `test_cases` folder

use anyhow::{Context, Result};

/// Generate tests from the *µcad* files in the given `path`
///
/// Path will be scanned recursively
pub fn generate(path: impl AsRef<std::path::Path>) -> Result<()> {
    use std::*;
    println!(
        "cargo:rerun-if-changed={path}",
        path = path.as_ref().display()
    );
    // get target path
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = path::Path::new(&out_dir).join("microcad_source_file_test.rs");

    // we will create a single output file whose content will be written into this variable first
    let mut code = String::from(
        "// This code was generated by microcad_source_file_test
// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

",
    );

    // read all *Markdown files and write result into `code`
    scan(&mut code, path.as_ref(), "µcad")?;

    // reformat code and write into file
    match rustfmt_wrapper::rustfmt(code) {
        Ok(code) =>
        // write all rust code at once
        {
            fs::write(&dest_path, code).context(format!("cannot create file '{dest_path:?}'"))?;
            Ok(())
        }
        Err(rustfmt_wrapper::Error::Rustfmt(msg)) => {
            Err(anyhow::Error::msg(msg.clone())).context(msg)
        }
        Err(err) => Err(anyhow::Error::new(err)),
    }
}

/// scan folder
fn scan(output: &mut String, path: &std::path::Path, extension: &str) -> Result<bool> {
    // prepare return value
    let mut found = false;
    // read given directory
    for entry in std::fs::read_dir(path)?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            let file_name = entry.file_name().into_string().unwrap();
            // check if directory or file
            if file_type.is_dir() {
                let mut code = String::new();
                // scan deeper
                if scan(&mut code, &entry.path(), extension)? {
                    if let Some(name) = entry.path().file_stem() {
                        let name = name.to_str().unwrap();
                        output.push_str(&format!(
                            "#[allow(non_snake_case)]
                             mod r#{name} {{
                                 {code}
                             }}\n\n"
                        ))
                    } else {
                        output.push_str(&code);
                    }

                    // found something
                    found = true;
                }
            } else if file_type.is_file() && file_name.ends_with(&format!(".{extension}")) {
                generate_source_file_test_code(output, &entry.path());
                // tell cargo to watch this file
                println!("cargo:rerun-if-changed={}", entry.path().display());
                // found something
                found = true;
            }
        }
    }

    Ok(found)
}

fn generate_source_file_test_code(output: &mut String, file_path: &std::path::Path) {
    let name = file_path.file_stem().unwrap().to_str().unwrap();

    let code = format!(
        r#"crate::test_source_file("{file_path}");"#,
        file_path = file_path
            .to_str()
            .unwrap()
            .escape_default() // Escape characters correctly (e.g. backslashes in Windows paths)
            .collect::<String>()
    );

    output.push_str(&format!(
        r##"#[test]
                        fn r#{name}() {{
                            {code}
                        }}"##,
    ));
}

#[test]
fn generate_source_file_tests() {
    std::env::set_var("OUT_DIR", "..");
    generate("../test_cases").unwrap();
}
