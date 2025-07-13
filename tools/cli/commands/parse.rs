// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command

use std::rc::Rc;

use crate::*;

use microcad_lang::{parse::ParseResult, syntax::SourceFile};

#[derive(clap::Parser)]
pub struct Parse {
    /// Input µcad file.
    pub input: std::path::PathBuf,
}

impl Parse {
    pub fn parse(&self) -> ParseResult<Rc<SourceFile>> {
        let source_file = SourceFile::load(self.input.clone())?;
        log::info!("Parsed successfully!");
        Ok(source_file)
    }
}

impl RunCommand for Parse {
    fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        self.parse()?;
        Ok(())
    }
}
