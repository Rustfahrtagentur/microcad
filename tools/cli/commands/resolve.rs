// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{diag::*, rc::*, resolve::*, syntax::*};

use crate::*;

#[derive(clap::Parser)]
pub struct Resolve {
    /// Input µcad file.
    pub input: std::path::PathBuf,
    /// Output symbol table.
    pub output: Option<std::path::PathBuf>,
}

impl Resolve {
    pub fn load(&self) -> ResolveResult<Rc<SourceFile>> {
        let source = crate::commands::parse::Parse {
            input: self.input.clone(),
        }
        .parse()?;
        log::info!("Resolved successfully!");
        Ok(source)
    }
}

impl RunCommand for Resolve {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let root = self.load()?;

        let mut context = match ResolveContext::load_and_resolve(
            root,
            &cli.search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        ) {
            Ok(symbol_table) => symbol_table,
            Err(err) => todo!(),
        };
        let unchecked = context.check()?;

        if context.has_errors() {
            print!("{}", context.diagnosis());
        }
        match &self.output {
            Some(filename) => {
                context.write_to_file(&filename)?;
                todo!("write unchecked into file");
            }
            None => println!("{context}"),
        }

        Ok(())
    }
}
