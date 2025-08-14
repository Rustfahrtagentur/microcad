// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{diag::*, resolve::*};

use crate::*;

#[derive(clap::Parser)]
pub struct Resolve {
    /// Input µcad file.
    pub input: std::path::PathBuf,
    /// Output symbol table.
    pub output: Option<std::path::PathBuf>,
}

impl Resolve {
    pub fn resolve(&self) -> ResolveResult<Symbol> {
        let symbol_node = crate::commands::parse::Parse {
            input: self.input.clone(),
        }
        .parse()?
        .resolve(None)?;
        log::info!("Resolved successfully!");
        Ok(symbol_node)
    }
}

impl RunCommand for Resolve {
    fn run(&self, _: &Cli) -> anyhow::Result<()> {
        let symbol = self.resolve()?;

        match &self.output {
            Some(filename) => symbol.write_to_file(&filename)?,
            None => println!("{symbol}"),
        }

        Ok(())
    }
}
