// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI build script.

use anyhow::{Context, Result};

use std::{env, fs, path::Path};
use walkdir::WalkDir;

/// Generate Rust HashMap that contain the standard library.
fn generate_builtin_std_library() -> Result<()> {
    // Helper function to escape quotes in strings
    fn escape_string(s: &str) -> String {
        s.replace('"', r#"\""#)
    }

    // Get the directory to scan from the environment variable,
    // or use a default (e.g., "assets" in your project root).
    let dir = env::var("MICROCAD_STD_DIR").unwrap_or_else(|_| "../../lib/std".to_string());
    let out_dir = env::var("OUT_DIR").expect("Some output dir");
    let dest_path = Path::new(&out_dir).join("microcad_std.rs");

    // Collect all .µcad files recursively
    let mut files = Vec::new();
    for entry in WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "µcad"))
    {
        let path = entry.path();
        let rel_path = path.strip_prefix(&dir).expect("Some prefix").to_path_buf();
        let content = fs::read_to_string(path).expect("Unable to read file");
        files.push((rel_path, content));
    }

    // Generate the Rust code
    let mut code = String::new();
    code.push_str(
        "
        mod microcad_std {

        use std::path::PathBuf;
        use std::collections::HashMap;

        lazy_static::lazy_static! {
            pub static ref FILES: HashMap<PathBuf, String> = {
                let mut m = HashMap::new();
    ",
    );

    for (path, content) in files {
        let path_str = path.to_string_lossy();
        let content = escape_string(&content);
        code.push_str(&format!(
            "        m.insert(PathBuf::from(r#\"{path_str}\"#), r#\"{content}\"#.to_string());\n"
        ));
    }

    code.push_str(
        "
                m
            };
        }
    }
    ",
    );
    // reformat code and write into file
    match rustfmt_wrapper::rustfmt(code) {
        Ok(code) =>
        // write all rust code at once
        {
            fs::write(&dest_path, code).context(format!("cannot create file '{dest_path:?}'"))?;
            println!("cargo:rerun-if-changed={dir}");
            Ok(())
        }
        Err(rustfmt_wrapper::Error::Rustfmt(msg)) => {
            Err(anyhow::Error::msg(msg.clone())).context(msg)
        }
        Err(err) => Err(anyhow::Error::new(err)),
    }
}

fn main() {
    generate_builtin_std_library().expect("No error");
}
