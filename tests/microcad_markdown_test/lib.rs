use anyhow::{Context, Result};
use tree::Tree;

mod tree;

pub fn generate(path: impl AsRef<std::path::Path>) -> Result<()> {
    use std::{env::*, fs::*, path::*};

    /// recursive directory scanner
    /// returns `false` if no code was generated
    fn recurse(tree: &mut Tree, path: &Path) -> Result<bool> {
        // prepare return value
        let mut found = false;
        // read given directory
        for entry in read_dir(path)?.flatten() {
            // get file type
            if let Ok(file_type) = entry.file_type() {
                let file_name = entry.file_name().into_string().unwrap();
                // check if directory or Markdown file
                if file_type.is_dir() && ![".", ".."].contains(&file_name.as_str()) {
                    // scan deeper
                    if recurse(tree, &entry.path())? {
                        // generated code
                        found = true;
                    }
                } else if file_type.is_file()
                    && file_name.ends_with(".md")
                    && !generate_tests_for_md_file(tree, &entry.path())?
                {
                    // tell cargo to watch this file
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                    // generated code
                    found = true;
                }
            }
        }
        Ok(found)
    }

    // get target path
    let out_dir = var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("microcad_markdown_test.rs");

    // read all into a tree to reorder modules
    let mut tree = Tree::new();
    recurse(&mut tree, path.as_ref())?;

    match rustfmt_wrapper::rustfmt(tree.to_string()) {
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

fn generate_tests_for_md_file(tree: &mut Tree, path: &std::path::Path) -> Result<bool> {
    use regex::*;
    use std::{fs::*, io::*};

    // load markdown file
    let mut md_content = String::new();
    {
        File::open(path)?.read_to_string(&mut md_content)?;
    }

    // match markdown code markers for µCAD
    let reg = Regex::new(r#"```µ[Cc][Aa][Dd](,(?<name>[\.#\w]+))?\n(?<code>[^`]*)+```"#)
        .expect("bad regex");

    let path = path
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
                tree.insert(name.as_str(), code.as_str().to_string());
            } else {
                tree.insert(
                    &format!("{path}.{}", name.as_str()),
                    code.as_str().to_string(),
                );
            }
            result = false;
        }
    }

    Ok(result)
}
