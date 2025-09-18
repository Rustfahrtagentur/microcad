// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use crate::*;

use anyhow::{Context, Result};

include!(concat!(env!("OUT_DIR"), "/microcad_std.rs"));

#[derive(clap::Parser)]
pub struct Install {
    /// Name of µcad library to install (currently only `std` is supported).
    pub library: String,

    /// Directory to install libraries into.
    /// If this command line option is not set, the library will be installed in ~/.microcad/lib/$LIBNAME
    pub root: Option<std::path::PathBuf>,

    /// Force overwrite.
    #[arg(short = 'f', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    force: bool,
}

impl Install {
    /// Get library dir in which the library is supposed to installed.
    pub fn library_dir(&self) -> std::path::PathBuf {
        match &self.root {
            Some(root) => root.clone(),
            // If root has not been passed as argument, install to home directory
            None => {
                let root_dir = Cli::global_root_dir().unwrap_or(std::path::PathBuf::from("./lib"));
                root_dir.join(self.library.clone())
            }
        }
    }

    pub fn install_std_library(&self) -> Result<()> {
        let library_dir = self.library_dir();
        log::info!("Install µcad library to {library_dir:?} ...");

        if !library_dir.exists() {
            log::debug!("Creating directory for µcad std library");
            std::fs::create_dir_all(library_dir.clone())?;
        } else if !self.force {
            return Err(anyhow::anyhow!(
                "The library seems to be installed already. Use `--force` to overwrite the existing installation."
            ));
        }

        for (filename, content) in microcad_std::FILES.iter() {
            let dest_path = library_dir.join(filename);
            // Ensure parent directories exist
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dest_path, content)
                .with_context(|| format!("Failed to write file: {}", dest_path.display()))?;
            log::trace!("Wrote µcad file: {dest_path:?}");
        }

        log::info!("Successfully installed µcad library to {library_dir:?}.");

        Ok(())
    }
}

impl RunCommand for Install {
    fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        if self.library == "std" {
            self.install_std_library()
        } else {
            Err(anyhow::anyhow!(
                "Only `std` is supported as installable library at the moment.",
            ))
        }
    }
}
