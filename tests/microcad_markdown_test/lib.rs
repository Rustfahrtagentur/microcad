// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(missing_docs)]

//! Generate tests out of *Markdown* files which include µCAD code
//!
//! Path will be scanned recursively for *Markdown* files (`*.md`).
//! Code must be marked by *Markdown* code markers (code type: `µCAD`) with a test ID attached.
//! In case of a failing test `#fail` must be appended to the test ID.
//!
//! Relative path's of scanned folder names will be used to build a modules structure  
//! in the resulting code.
//! If test IDs include `.` name will be split into several names which will be
//! used to crates sub modules.
use anyhow::{Context, Result};
use walk_path::*;

/// Generate tests from the *Markdown* files which are within the given `path`
///
/// Path will be scanned recursively
pub fn generate(path: impl AsRef<std::path::Path>) -> Result<()> {
    use std::{env::*, path::*};

    // get target path
    let out_dir = var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("microcad_markdown_test.rs");

    // read all *Markdown files into a tree to reorder modules
    let mut wp = WalkPath::new();
    wp.scan(path.as_ref(), "md", &scan_for_tests)?;

    let mut code = String::new();
    write(&mut code, &wp);

    match rustfmt_wrapper::rustfmt(code) {
        Ok(code) =>
        // write all rust code at once
        {
            std::fs::write(dest_path, code)
                .context("cannot create file 'microcad_markdown_test.rs'")?;
            Ok(())
        }
        Err(rustfmt_wrapper::Error::Rustfmt(msg)) => {
            Err(anyhow::Error::msg(msg.clone())).context(msg)
        }
        Err(err) => Err(anyhow::Error::new(err)),
    }
}

/// Read single *Markdown* file and collect included tests in `tree`.
///
/// Generates tree nodes if name can be split into several names which are separated by `.`.
fn scan_for_tests(tree: &mut WalkPath<String>, file_path: &std::path::Path) -> Result<bool> {
    use regex::*;
    use std::{fs::*, io::*};

    // load markdown file
    let mut md_content = String::new();
    {
        File::open(file_path)?.read_to_string(&mut md_content)?;
    }

    // match markdown code markers for µCAD
    let reg = Regex::new(r#"```µ[Cc][Aa][Dd](,(?<name>[\.#\w]+))?[\r\n]+(?<code>[^`]*)+```"#)
        .expect("bad regex");

    let path = file_path
        .iter()
        .map(|f| f.to_str().unwrap())
        .filter(|f| *f != "..")
        .map(|f| *f.split('.').next().as_ref().unwrap())
        .collect::<Vec<&str>>()
        .join(".");

    let mut result = true;
    for cap in reg.captures_iter(&md_content) {
        // check if code is named

        if let (Some(code), Some(name)) = (cap.name("code"), cap.name("name")) {
            if path.is_empty() {
                insert(tree, name.as_str(), code.as_str().to_string());
            } else {
                insert(
                    tree,
                    &format!("{path}.{}", name.as_str()),
                    code.as_str().to_string(),
                );
            }
            result = false;
        }
    }

    Ok(result)
}

/// insert new test code by module path
/// - `path`: list of nested rust module names separated by `.`
/// - `code`: µCAD test code
fn insert(wp: &mut WalkPath<String>, path: &str, code: String) {
    use std::{cell::RefCell, rc::Rc};

    if let Some((path, crumbs)) = path.split_once('.') {
        match wp {
            WalkPath::Root(ref mut children) | WalkPath::Dir(_, ref mut children) => {
                if let Some(ref mut file) = children.get(std::path::Path::new(path)) {
                    insert(&mut file.borrow_mut(), crumbs, code);
                } else {
                    _ = children.insert(path.into(), {
                        let mut new = WalkPath::Dir(path.into(), Dir::<String>::new());
                        // recursively fill module
                        insert(&mut new, crumbs, code);
                        Rc::new(RefCell::new(new))
                    })
                }
            }
            _ => unreachable!(),
        }
    } else {
        match wp {
            WalkPath::Dir(_, ref mut children) | WalkPath::Root(ref mut children) => {
                _ = children.insert(
                    path.into(),
                    Rc::new(RefCell::new(WalkPath::File(path.into(), code))),
                )
            }
            _ => unreachable!(),
        }
    }
}

fn write(f: &mut String, wp: &WalkPath<String>) {
    match wp {
        WalkPath::Root(children) => {
            for child in children {
                f.push_str(
                    "// This code was generated by microcad_markdown_test
                    
                    ",
                );
                write(f, &child.1.as_ref().borrow());
            }
        }
        WalkPath::Dir(name, children) => {
            let name = name.to_str().unwrap();
            f.push_str(&format!(
                "mod r#{name} {{
                #![allow(non_snake_case)]
                
            "
            ));
            for child in children {
                write(f, &child.1.as_ref().borrow());
            }
            f.push_str("}\n\n");
        }
        WalkPath::File(name, code) => {
            let name = name.to_str().unwrap();
            let (name, suffix) = if let Some((name, suffix)) = name.split_once('#') {
                (name, Some(suffix))
            } else {
                (name, None)
            };
            // Early exit for "#no_test" and "#todo" suffixes
            if suffix == Some("no_test") || suffix == Some("todo") {
                return;
            }
            f.push_str(
                &format!(
                    r##"#[test]
                        fn r#{name}() {{
                            use microcad_lang::{{eval::{{Symbols, Eval, Context}},parse::source_file::SourceFile,parser}};
                            match SourceFile::load_from_str(
                                r#"
                                {code}"#,
                            ) {handling};
                        }}"##,
                    handling = match suffix {
                        Some("fail") =>
                            r##"{
                                    Err(_) => (),
                                    Ok(source) => { 
                                        let mut context = Context::default();
                                        context.add(microcad_std::builtin_module().into());

                                        if let Err(err) = source.eval(&mut context) {
                                            println!("{err}");
                                        } else {
                                            panic!("ERROR: test is marked to fail but succeeded");
                                        }
                                    }
                                }"##,
                        _ =>
                            r##"{
                                    Ok(source) => {
                                        let mut context = Context::default();
                                        context.add(microcad_std::builtin_module().into());
                                        
                                        if let Err(err) = source.eval(&mut context) {
                                            println!("{err}");
                                        }
                                    },
                                    Err(err) => panic!("ERROR: {err}"),
                                }"##,
                    }
                )
            );
        }
    }
}
