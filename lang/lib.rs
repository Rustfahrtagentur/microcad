// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Processing of µcad source code.
//!
//! This module includes all components to parse, resolve and evaluate µcad code.and diagnose errors.
//!
//! - Load and parse source files in [`parse`] and [`syntax`]
//! - Resolve parsed sources in [`resolve`]
//! - Evaluate resolved sources in [`eval`]
//! - Diagnose any evaluation errors in [`diag`]
//!
//! The grammar of µcad can be found [here](../../../lang/grammar.pest).

pub mod builtin;
pub mod diag;
pub mod eval;
pub mod modeltree;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod rc;
pub mod resolve;
pub mod src_ref;
pub mod syntax;
pub mod ty;
pub mod value;

/// Id type (base of all identifiers)
type Id = compact_str::CompactString;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

const MICROCAD_EXTENSIONS: &[&str] = &[".µcad", ".mcad"];
