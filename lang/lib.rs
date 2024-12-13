// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD source code parser

#![warn(missing_docs)]

pub mod diag;
pub mod eval;
pub mod map_key_type;
pub mod objecttree;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod source_file_cache;
pub mod src_ref;
pub mod r#type;

use std::sync::Once;

pub use objecttree::{ObjectNode, ObjectNodeInner};

static INIT_EVENT_LOGGER: Once = Once::new();

/// Initialize env_logger
pub fn env_logger_init() {
    INIT_EVENT_LOGGER.call_once(env_logger::init);
}
