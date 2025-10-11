// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command.

use crate::*;

use microcad_lang::{rc::*, syntax::*, tree_display::*};

#[derive(clap::Parser)]
pub struct Parse {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Print syntax tree.
    #[clap(long)]
    pub syntax: bool,
}

impl RunCommand<Rc<SourceFile>> for Parse {
    fn run(&self, _cli: &Cli) -> anyhow::Result<Rc<SourceFile>> {
        let source_file = SourceFile::load(self.input.clone())?;
        eprintln!("Parsed successfully!");
        if self.syntax {
            println!("{}", FormatTree(source_file.as_ref()));
        }
        Ok(source_file)
    }
}
