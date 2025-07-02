// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI create command

use crate::*;

use std::io::Write;

#[derive(clap::Parser)]
pub struct Create {
    path: std::path::PathBuf,
}

impl RunCommand for Create {
    fn run(&self, _cli: &crate::cli::Cli) -> anyhow::Result<()> {
        let mut path = self.path.to_path_buf();
        if path.extension().is_none() {
            path.set_extension("µcad");
        }

        if path.exists() {
            eprintln!("Error: File {path:?} already exists.")
        } else {
            // create demo program
            let mut f = std::fs::File::create(path.clone())?;
            f.write_all(
                r#"// µcad generated file
use std::*;

sketch main() {
  print("Hello µcad");
}

main();
"#
                .as_bytes(),
            )?;
            eprintln!("File {path:?} generated.")
        }

        Ok(())
    }
}
