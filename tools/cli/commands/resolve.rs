// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{parse::ParseResult, resolve::Symbol};

use crate::*;

#[derive(clap::Parser)]
pub struct Resolve {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl Resolve {
    pub fn resolve(&self) -> ParseResult<Symbol> {
        let symbol_node = crate::commands::parse::Parse {
            input: self.input.clone(),
        }
        .parse()?
        .resolve(None);
        log::info!("Resolved successfully!");
        Ok(symbol_node)
    }
}

impl RunCommand for Resolve {
    fn run(&self, _: &Cli) -> anyhow::Result<()> {
        self.resolve()?;
        Ok(())
    }
}
