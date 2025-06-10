// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code parsing
//!
//! A source file on disc is just a bunch of UTF-8 encoded text which must be parsed
//! before any processing:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*};
//!
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//! ```
//!
//! To read a source file from an already loaded string use:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*};
//!
//! let source_file = SourceFile::load_from_str(r#"std::print("hello world!");"#).expect("parsing success");
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be resolved and evaluated (see [`crate::resolve`] and [`crate::eval`])  .

mod attribute;
mod body;
mod call;
mod expression;
mod format_string;
mod function;
mod identifier;
mod lang_type;
mod literal;
mod module;
mod parameter;
mod part;
mod source_file;
mod statement;
mod r#type;
mod r#use;

pub mod parse_error;

pub use parse_error::*;

use crate::{src_ref::*, syntax::*, ty::*};
const INTERNAL_PARSE_ERROR: &str = "internal parse error";
