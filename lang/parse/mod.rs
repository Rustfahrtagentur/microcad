// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source code parse entities

mod body;
mod call;
mod expression;
mod format_string;
mod function;
mod identifier;
mod lang_type;
mod literal;
mod module;
mod namespace;
mod parameter;
mod source_file;
mod statement;
mod r#type;
mod r#use;

pub mod parse_error;

pub use parse_error::*;

use crate::{src_ref::*, syntax::*, r#type::*};

const INTERNAL_PARSE_ERROR: &str = "internal parse error";
