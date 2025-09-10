// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad inspector

extern crate microcad_lang;

use clap::Parser;

use microcad_lang::eval::Context;
use microcad_lang::resolve::FullyQualify;
use microcad_lang::syntax::*;

use std::rc::Rc;
use std::sync::mpsc;
mod watcher;

use microcad_lang::model::Model;

use slint::VecModel;

use crate::watcher::Watcher;
slint::include_modules!();

#[derive(Parser)]
struct Args {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Paths to search for files.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

struct Inspector {
    args: Args,

    pub watcher: Watcher,
}

impl VM_Item {
    fn _from_model(model: &Model, items: &mut Vec<Self>, depth: usize) {
        let model_ = model.borrow();

        let creator = match model_.element.creator() {
            Some(creator) => VM_Creator {
                symbol_name: creator.symbol.full_name().to_string().into(),
                //   arguments: slint::ModelRc::new(VecModel::from(vec![])),
            },
            None => VM_Creator::default(),
        };

        items.push(VM_Item {
            depth: depth as i32,
            element: model_.element.value.to_string().into(),
            src_ref: model_.element.src_ref.to_string().into(),
            creator,
        });
        model_
            .children()
            .for_each(|model| Self::_from_model(model, items, depth + 1))
    }

    /// Create ViewModelItems from model, including children
    pub fn from_model(model: &Model) -> Vec<Self> {
        let mut items = Vec::new();
        Self::_from_model(model, &mut items, 0);
        items
    }
}

impl Inspector {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            args: Args::parse(),
            watcher: Watcher::new()?,
        })
    }

    fn load(&self) -> anyhow::Result<Rc<SourceFile>> {
        let source = SourceFile::load(self.args.input.clone())?;
        log::info!("Resolved successfully!");
        Ok(source)
    }

    /// Make a new context from an input file.
    fn make_context(&self) -> anyhow::Result<Context> {
        Ok(microcad_builtin::builtin_context(
            self.load()?,
            &self.args.search_paths,
        )?)
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        // Create the Slint UI component
        let main_window = MainWindow::new()?;

        let weak = main_window.as_weak();

        std::thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let (tx, rx): (mpsc::Sender<Vec<VM_Item>>, _) = mpsc::channel();
                // Watch all dependencies of the most recent compilation.
                self.watcher.update(vec![self.args.input.clone()])?;

                // Create a vector of model items
                let items = match self.make_context() {
                    Ok(mut context) => {
                        // Re-evaluate context.
                        match context.eval() {
                            Ok(model) => {
                                // Model
                                // println!("{}", FormatTree(&model));
                                VM_Item::from_model(&model)
                            }
                            Err(err) => {
                                log::error!("{err}");
                                vec![]
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("{err}");
                        vec![]
                    }
                };

                // Wait until anything relevant happens.
                tx.send(items)?;

                weak.upgrade_in_event_loop(move |main_window| {
                    let items = rx.recv().expect("No error");
                    let view_model = VecModel::from(items);
                    main_window.set_view_model(slint::ModelRc::new(view_model))
                })?;

                self.watcher.wait()?;
            }
        });

        main_window.run()?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    Inspector::new()?.run()
}
