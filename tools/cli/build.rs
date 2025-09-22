// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI build script.

use anyhow::{Context, Result};

use std::{env, fs, path::Path};
use walkdir::WalkDir;

/// for debugging purpose
#[allow(unused)]
macro_rules! warning {
    ($($tokens: tt)*) => {
        // HINT: switch `note` -> `warning` to activate debug messages
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

/// Generate Rust HashMap that contain the standard library.
fn generate_builtin_std_library() -> Result<()> {
    use std::path::PathBuf;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("Manifest dir");

    let dir = env::var("MICROCAD_STD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("lib/std"));

    let dest_path = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("Output dir")
        .join("microcad_std.rs");

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
            println!("cargo:rerun-if-changed={dir}", dir = dir.display());
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
