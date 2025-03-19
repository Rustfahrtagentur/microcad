// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source code parse entities

pub mod body;
pub mod call;
pub mod expression;
pub mod format_string;
pub mod function;
pub mod identifier;
pub mod lang_type;
pub mod literal;
pub mod module;
pub mod namespace;
pub mod parameter;
pub mod parse_error;
pub mod source_file;
pub mod statement;
pub mod r#use;

pub use body::*;
pub use call::*;
pub use expression::*;
pub use format_string::*;
pub use function::*;
pub use identifier::*;
pub use lang_type::*;
pub use literal::*;
pub use module::*;
pub use namespace::*;
pub use parameter::*;
pub use parse_error::*;
pub use r#use::*;
pub use source_file::*;
pub use statement::*;

const INTERNAL_PARSE_ERROR: &str = "internal parse error";

/// Trait for printing a syntax tree
pub trait PrintSyntax {
    /// Print a syntax tree
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result;
}
