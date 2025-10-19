// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command

use microcad_lang::{model::Model, rc::RcMut, render::*, value::Value};

use crate::*;

#[derive(clap::Parser)]
pub struct Watch {
    /// Export arguments.
    #[clap(flatten)]
    pub export: Export,
}

/// Run this command for a CLI.
impl RunCommand for Watch {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut watcher = Watcher::new()?;
        let render_cache = RcMut::new(RenderCache::default());

        if !self.export.dry_run {
            // Recompile whenever something relevant happens.
            loop {
                // run prior parse step
                match self.export.run(cli) {
                    Ok(target_models) => {
                        target_models.iter().try_for_each(
                            |(model, export)| -> anyhow::Result<()> {
                                let mut render_context = RenderContext::init(
                                    model,
                                    self.export.resolution(),
                                    Some(render_cache.clone()),
                                )?;
                                let model: Model =
                                    model.render_with_context(&mut render_context)?;

                                let value = export.export(&model)?;
                                if !matches!(value, Value::None) {
                                    log::info!("{value}");
                                };
                                Ok(())
                            },
                        )?;
                    }
                    Err(err) => log::error!("{err}"),
                }

                // Watch all dependencies of the most recent compilation.
                watcher.update(vec![self.export.eval.resolve.parse.input.clone()])?;

                // Remove unused cache items.
                {
                    let mut cache = render_cache.borrow_mut();
                    cache.gc();
                }

                // Wait until anything relevant happens.
                watcher.wait()?;
            }
        }

        Ok(())
    }
}
