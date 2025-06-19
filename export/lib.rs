// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export model nodes to files  

pub mod ply;
pub mod stl;
pub mod svg;

use microcad_lang::model_tree::ModelNode;
use thiserror::Error;

/// Export error stub.
#[derive(Error, Debug)]
pub enum ExportError {}

/// Exporter trait
pub trait Exporter {
    /// Return file extensions
    fn file_extensions(&self) -> Vec<&str>;

    /// Do export
    fn export(&mut self, node: ModelNode) -> Result<(), ExportError>;
}
