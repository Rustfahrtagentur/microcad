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

use crate::{syntax::Identifier, value::Value};

pub mod builtin;
pub mod diag;
pub mod eval;
pub mod model_tree;
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
pub type Id = compact_str::CompactString;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

const MICROCAD_EXTENSIONS: &[&str] = &[".µcad", ".mcad"];

/// Access a value of a property by id.
pub trait GetPropertyValue {
    /// Get a value of property, or [`Value::None`] if the property does not exist.
    ///
    /// - `id`: Identifier of the property
    ///
    /// This function is used when accessing a property `v` of a value `p` with `p.v`.
    fn get_property_value(&self, id: &Identifier) -> Value;
}

/// Parse a rule from given string into a syntax element
/// - `ty`: Type of the output syntax element
/// - `rule`: Parsing rule to use.
/// - `code`: String slice of the code to parse
#[macro_export]
macro_rules! parse {
    ($ty:path, $rule:path, $code:expr) => {
        $crate::parser::Parser::parse_rule::<$ty>($rule, $code, 0).expect("bad inline code")
    };
}

#[test]
fn parse_macro() {
    let y3 = 3;
    let p = parse!(
        syntax::ParameterList,
        parser::Rule::parameter_list,
        &format!("(x=0,y=[1,2,{y3},4],z=2)")
    );
    assert_eq!(p.to_string(), "x = 0, y = [1, 2, 3, 4], z = 2");
}

/// Shortens given string to iz's first line and to MAX_LEN characters
pub fn shorten(what: &str, max_chars: usize) -> String {
    what.chars()
        .enumerate()
        .filter_map(|(p, ch)| {
            if p == max_chars {
                Some('…')
            } else if p < max_chars {
                if ch == '\n' {
                    Some('⏎')
                } else {
                    Some(ch)
                }
            } else {
                None
            }
        })
        .collect()
}

/// Shortens given string to iz's first line and to MAX_LEN characters with default maximum length
#[macro_export]
macro_rules! shorten {
    ($what:expr) => {
        $crate::shorten(&format!("{}", $what), 80)
    };
    ($what:expr, $max_chars:literal) => {
        shorten(format!("{}", $what).lines(), max_chars)
    };
}
