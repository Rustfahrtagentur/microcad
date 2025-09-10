// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad inspector

use clap::Parser;

#[derive(Parser)]
struct Args {
    input: std::path::PathBuf,
}

use slint::{Model, VecModel};
slint::include_modules!();

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    // Create a vector of model items
    let data = vec![
        ModelItem {
            name: "Item 1".into(),
        },
        ModelItem {
            name: "Item 2".into(),
        },
        ModelItem {
            name: "Item 3".into(),
        },
    ];

    // Wrap in a VecModel
    let model = VecModel::from(data);

    // Create the Slint UI component
    let main_window = MainWindow::new()?;
    main_window.set_model(slint::ModelRc::new(model));
    main_window.run()?;

    Ok(())
}
