// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests out of *Markdown* files which include µcad code
//!
//! Path will be scanned recursively for *Markdown* files (`*.md`).
//! Code must be marked by *Markdown* code markers (code type: `µcad`) with a test ID attached.
//! In case of a failing test `#fail` must be appended to the test ID.
//!
//! Relative path's of scanned folder names will be used to build a modules structure  
//! in the resulting code.
//! If test IDs include `.` name will be split into several names which will be
//! used to crates sub modules.

use anyhow::{Context, Result};

/// for debugging purpose
#[allow(unused)]
macro_rules! warning {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

#[test]
fn md_tests() {
    generate("..").unwrap();
}

/// Generate tests from the *Markdown* files which are within the given `path`
///
/// Path will be scanned recursively
pub fn generate(path: impl AsRef<std::path::Path>) -> Result<()> {
    use std::*;

    // get target path
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = path::Path::new(&out_dir).join("microcad_markdown_test.rs");

    // we will create a single output file whose content will be written into this variable first
    let mut code = String::from(
        "// This code was generated by microcad_markdown_test
// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later
static SEARCH_PATH: &str = \"../std\";
",
    );

    // directories to exclude
    let exclude_dirs = ["target", "thirdparty"];

    // remove any previous banners
    remove_banner(path.as_ref(), &exclude_dirs)?;

    // read all *Markdown files and write result into `code`
    scan(&mut code, path.as_ref(), "md", &exclude_dirs)?;

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

/// Remove all banners in `path` and exclude folders whose names are contained
/// in `exclude_dirs` from search.
fn remove_banner(path: impl AsRef<std::path::Path>, exclude_dirs: &[&str]) -> Result<()> {
    for entry in std::fs::read_dir(&path)?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            // check if directory or file
            if file_type.is_dir()
                && !exclude_dirs.contains(&entry.file_name().to_string_lossy().to_string().as_str())
            {
                if entry.file_name() == ".banner" {
                    remove_dir(entry.path())?;
                } else {
                    remove_banner(entry.path(), exclude_dirs)?;
                }
            }
        }
    }

    Ok(())
}

/// Remove all files within `.banner` directory
fn remove_dir(path: impl AsRef<std::path::Path>) -> Result<()> {
    warning!("remove banners in: {:?}", path.as_ref());

    // list all files within `.banner` directory and remove them
    for entry in std::fs::read_dir(&path)
        .unwrap()
        .flatten()
        .filter(|entry| entry.file_type().unwrap().is_file())
    {
        std::fs::remove_file(entry.path())?;
    }
    Ok(())
}

/// scan folder
fn scan(
    output: &mut String,
    path: &std::path::Path,
    extension: &str,
    exclude_dir: &[&str],
) -> Result<bool> {
    // prepare return value
    let mut found = false;
    // read given directory
    for entry in std::fs::read_dir(path)?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            let file_name = entry.file_name().into_string().unwrap();
            // check if directory or file
            if file_type.is_dir() && !exclude_dir.contains(&file_name.as_str()) {
                let mut code = String::new();
                // scan deeper
                if scan(&mut code, &entry.path(), extension, exclude_dir)? {
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
            } else if file_type.is_file()
                && file_name.ends_with(&format!(".{extension}"))
                && !scan_for_tests(output, &entry.path())?
            {
                // tell cargo to watch this file
                println!("cargo:rerun-if-changed={}", entry.path().display());
                // found something
                found = true;
            }
        }
    }
    Ok(found)
}

/// Read single *Markdown* file and collect included tests in `tree`.
///
/// Generates tree nodes if name can be split into several names which are separated by `.`.
fn scan_for_tests(output: &mut String, file_path: &std::path::Path) -> Result<bool> {
    use regex::*;
    use std::{fs::*, io::*};

    // `true`` if we didn't found anything
    let mut result = true;

    // load markdown file
    let mut md_content = String::new();
    {
        File::open(file_path)?.read_to_string(&mut md_content)?;
    }

    // accumulate name and code while reading file
    let mut test_name = String::new();
    let mut test_code = String::new();

    // read all lines in the file
    for line in md_content.lines() {
        // match code starting marker
        if let Some(start) = Regex::new(r#"```µ[Cc][Aa][Dd](,(?<name>[\.#_\w]+))?"#)
            .expect("bad regex")
            .captures_iter(line)
            .next()
        {
            if let Some(name) = start.name("name") {
                // remember test name
                test_name = name.as_str().to_string();
                // clear code
                test_code.clear();
            }
        } else if !test_name.is_empty() {
            if Regex::new(r#"```"#) // match code end marker
                .expect("bad regex")
                .captures_iter(line)
                .next()
                .is_some()
            {
                warning!(
                    "scan_write_test_code: {file_path:?}, {}, {}",
                    test_name.as_str(),
                    test_code.as_str(),
                );

                // generate test code
                write_test_code(output, file_path, test_name.as_str(), test_code.as_str());

                // clear name to signal new test awaited
                test_name.clear();

                // found some test
                result = false;
            } else {
                // add line to code
                test_code.push_str(line);
                test_code.push('\n');
            }
        }
    }
    Ok(result)
}

/// Generate code for one test
fn write_test_code(f: &mut String, file_path: &std::path::Path, name: &str, code: &str) {
    // split name into `name` and `mode``
    let (name, mode) = if let Some((name, mode)) = name.split_once('#') {
        (name, Some(mode))
    } else {
        (name, None)
    };

    // where to store images
    let banner_path = file_path.parent().unwrap().join(".banner");
    // banner image file of this test
    let banner = banner_path
        .join(format!("{name}.png"))
        .to_string_lossy()
        .to_string();

    //warning!("write_test_code: banner: {banner} {:?}", file_path,);

    // maybe create .banner directory
    let _ = std::fs::create_dir(banner_path);

    // Early exit for "#no_test" and "#todo" suffixes
    match mode {
        Some("no_test") => return,
        Some("todo") => {
            let _ = std::fs::hard_link("images/todo.png", banner);
            return;
        }
        _ => (),
    };

    f.push_str(
    &format!(
        r##"#[test]
            #[allow(non_snake_case)]
            fn r#{name}() {{
                microcad_lang::env_logger_init();
                use microcad_lang::parse::*;
                use microcad_std::*;
                use crate::SEARCH_PATH;
                let banner = "{banner}";
                match SourceFile::load_from_str(
                    r#"
                    {code}"#,
                ) {handling};
            }}"##,
        handling = match mode {
            Some("fail") =>
                r##"{
                        Err(err) => {
                            let _ = std::fs::hard_link("images/fails.png", banner);

                            log::debug!("{err}")
                        },
                        Ok(source) => { 
                            let _ = std::fs::hard_link("images/succeeds.png", banner);

                            let mut context = ContextBuilder::new(source).with_std(SEARCH_PATH).build();
                            if let Err(err) = context.eval() {
                                log::debug!("{err}");
                            } else if context.diag().error_count > 0 {
                                let mut w = std::io::stdout();
                                context.diag().pretty_print(&mut w, &context).expect("internal error");
                            } else {
                                panic!("ERROR: test is marked to fail but succeeded");
                            }
                        }
                    }"##,
            _ =>
                r##"{
                        Ok(source) => {
                            let mut context = ContextBuilder::new(source).with_std(SEARCH_PATH).build();
                            if let Err(err) = context.eval() {
                                let _ = std::fs::hard_link("images/failing.png", banner);
                                panic!("{err}");
                            } else {
                                if context.diag().error_count > 0 {
                                    let _ = std::fs::hard_link("images/failing.png", banner);
                                    let mut w = std::io::stderr();
                                    context.diag().pretty_print(&mut w, &context).expect("internal error");
                                    panic!("ERROR: there were {error_count} errors", error_count = context.diag().error_count);
                                }
                                log::trace!("test succeeded");
                                let _ = std::fs::hard_link("images/success.png", banner);
                            }
                        },
                        Err(err) => {
                            let _ = std::fs::hard_link("images/failing.png", banner);

                            panic!("ERROR: {err}")
                        },
                    }"##,
        }
    )
);
}
