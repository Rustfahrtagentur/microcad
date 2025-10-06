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
    /// Skip checking all symbols after resolve.
    #[clap(short, long)]
    pub skip_check: bool,
}

impl Resolve {
    pub fn load(
        &self,
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> ResolveResult<ResolveContext> {
        //  parse root file
        let root = crate::commands::parse::Parse {
            input: self.input.clone(),
        }
        .parse()?;

        // resolve the file
        let mut context = ResolveContext::create(
            root,
            search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        )?;

        if !self.skip_check {
            context.check()?;
        }

        if context.has_errors() {
            eprint!("{}", context.diagnosis());
        }

        match &self.output {
            Some(filename) => {
                context.write_to_file(&filename)?;
                todo!("write unchecked into file");
            }
            None => print!("{context}"),
        }

        log::info!("Resolved successfully!");
        Ok(context)
    }
}

impl RunCommand for Resolve {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        self.load(&cli.search_paths)?;
        Ok(())
    }
}
