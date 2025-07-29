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
pub mod init_definition;
pub mod literal;
pub mod module;
pub mod parameter;
pub mod qualifier;
pub mod source_file;
pub mod statement;
pub mod type_annotation;
pub mod r#use;
pub mod visibility;
pub mod workbench;

pub use assignment::*;
pub use attribute::*;
pub use body::*;
pub use call::*;
pub use expression::*;
pub use format_string::*;
pub use function::*;
pub use identifier::*;
pub use init_definition::*;
pub use literal::*;
pub use module::*;
pub use parameter::*;
pub use qualifier::*;
pub use r#use::*;
pub use source_file::*;
pub use statement::*;
pub use type_annotation::*;
pub use visibility::*;
pub use workbench::*;

/// Trait for printing a syntax tree
pub trait PrintSyntax {
    /// Indention size
    const INDENT: usize = 2;
    /// Print a syntax tree
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result;
}
