// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements.
//!
//! Every element in the µcad language are parsed into definitions in this module.

pub mod assignment;
pub mod attribute;
pub mod body;
pub mod call;
pub mod expression;
pub mod format_string;
pub mod function;
pub mod identifier;
pub mod lang_type;
pub mod literal;
pub mod namespace;
pub mod parameter;
pub mod part;
pub mod source_file;
pub mod statement;
pub mod r#use;

pub use assignment::*;
pub use attribute::*;
pub use body::*;
pub use call::*;
pub use expression::*;
pub use format_string::*;
pub use function::*;
pub use identifier::*;
pub use lang_type::*;
pub use literal::*;
pub use namespace::*;
pub use parameter::*;
pub use part::*;
pub use r#use::*;
pub use source_file::*;
pub use statement::*;

/// Trait for printing a syntax tree
pub trait PrintSyntax {
    /// Print a syntax tree
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result;
}
