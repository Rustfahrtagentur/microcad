// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source code parser

pub mod diag;
pub mod objects;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod source_file_cache;
pub mod src_ref;
pub mod r#type;

pub mod argument_map;
pub mod ty;
pub mod value;

use std::sync::Once;

/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

static INIT_EVENT_LOGGER: Once = Once::new();

/// Initialize env_logger
pub fn env_logger_init() {
    INIT_EVENT_LOGGER.call_once(env_logger::init);
}
