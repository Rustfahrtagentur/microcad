// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{diag::*, resolve::*};

use crate::*;

#[derive(clap::Parser)]
pub struct Resolve {
    #[clap(flatten)]
    pub parse: Parse,

    /// Check all symbols after resolve.
    #[clap(short, long)]
    pub check: bool,

    /// Print resolve context.
    #[clap(long)]
    pub resolve: bool,
}

impl RunCommand<ResolveContext> for Resolve {
    fn run(&self, cli: &Cli) -> anyhow::Result<ResolveContext> {
        // run prior parse step
        let root = self.parse.run(cli)?;

        // resolve the file
        let mut context = ResolveContext::create(
            root,
            &cli.search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
            ResolveMode::Resolved,
        )?;

        if self.check {
            context.check()?;
        }

        if context.has_errors() {
            eprint!("{}", context.diagnosis());
        }

        if self.resolve {
            print!("{context}");
        }

        log::info!("Resolved successfully!");
        Ok(context)
    }
}
