// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Processing of µcad source code.
//!
//! This module includes all components to parse, resolve and evaluate µcad code and diagnose errors.
//!
//! - Load and parse source files in [`mod@parse`] and [`syntax`]
//! - Resolve parsed sources in [`resolve`]
//! - Evaluate resolved sources in [`eval`]
//! - Diagnose any evaluation errors in [`diag`]
//!
//! The grammar of µcad can be found [here](../../../lang/grammar.pest).
//!
//! Good starting point to understand how µcad syntax works: [`syntax::SourceFile::load()`] loads a µcad source file.

pub mod builtin;
pub mod diag;
pub mod eval;
pub mod model;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod rc;
pub mod render;
pub mod resolve;
pub mod src_ref;
pub mod syntax;
pub mod tree_display;
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

const MICROCAD_EXTENSIONS: &[&str] = &["µcad", "mcad", "ucad"];

/// Parse a rule from given string into a syntax element.
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

/// Shortens given string to it's first line and to `max_chars` characters.
pub fn shorten(what: &str, max_chars: usize) -> String {
    let short: String = what
        .chars()
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
        .collect();

    if cfg!(feature = "ansi-color") && short.contains('\x1b') {
        short + "\x1b[0m"
    } else {
        short
    }
}

/// Shortens given string to it's first line and to maximum characters.
#[macro_export]
macro_rules! shorten {
    ($what:expr) => {
        $crate::shorten(&format!("{}", $what), 140)
    };
    ($what:expr,$shorten:expr) => {
        if $shorten {
            $crate::shorten!($what)
        } else {
            $what
        }
    };
    ($what:expr, $max_chars:literal) => {
        shorten(format!("{}", $what).lines(), max_chars)
    };
}

/// Create a marker string which is colored with ANSI.
#[cfg(feature = "ansi-color")]
#[macro_export]
macro_rules! mark {
    (FOUND) => {
        color_print::cformat!("<W!,k,s> FOUND </>")
    };
    (FOUND_INTERIM) => {
        color_print::cformat!("<Y!,k,s> FOUND </>")
    };
    (FOUND_FINAL) => {
        color_print::cformat!("<G!,k,s> FOUND </>")
    };
    (MATCH) => {
        color_print::cformat!("<Y!,k,s> MATCH </>")
    };
    (CALL) => {
        color_print::cformat!("<B,k,s> CALL </>")
    };
    (LOAD) => {
        color_print::cformat!("<Y,k,s> LOADING </>")
    };
    (RESOLVE) => {
        color_print::cformat!("<M,k,s> RESOLVE </>")
    };
    (AMBIGUOUS) => {
        color_print::cformat!("<R,k,s> AMBIGUOUS </>")
    };
    (NOT_FOUND) => {
        color_print::cformat!("<R,k,s> NOT FOUND </>")
    };
    (NOT_FOUND_INTERIM) => {
        color_print::cformat!("<Y,k,s> NOT FOUND </>")
    };
}

#[cfg(not(feature = "ansi-color"))]
#[macro_export]
macro_rules! found {
    (FOUND) => {
        "Found"
    };
    (FINAL) => {
        "Found"
    };
    (INTERMEDIATE) => {
        "Found"
    };
    (MATCH) => {
        "Match"
    };
    (CALL) => {
        "Call"
    };
    (LOAD) => {
        "Loading"
    };
    (RESOLVE) => {
        "Resolve"
    };
    (AMBIGUOUS) => {
        "Ambiguous"
    };
    (NOT_FOUND) => {
        "Not found"
    };
    (NOT_FOUND_INTERIMEDIATE) => {
        "Not found"
    };
}

/// Generate string literal ` INVALID `*XXX*` ` with ANSI color.
#[cfg(feature = "ansi-color")]
#[macro_export]
macro_rules! invalid {
    (VALUE) => {
        color_print::cstr!("<R!,k,s> INVALID VALUE </>")
    };
    (TYPE) => {
        color_print::cstr!("<R!,k,s> INVALID TYPE </>")
    };
    (OUTPUT) => {
        color_print::cstr!("<R!,k,s> INVALID OUTPUT </>")
    };
    (STACK) => {
        color_print::cstr!("<W,k,s> EMPTY STACK </>")
    };
    (REF) => {
        color_print::cstr!("<Y!,k,s> NO REF </>")
    };
    (FILE) => {
        color_print::cstr!("<Y!,k,s> NO FILE </>")
    };
    (RESULT) => {
        color_print::cstr!("<Y!,k,s> NO RESULT </>")
    };
    (LINE) => {
        color_print::cstr!("<Y!,k,s> NO LINE </>")
    };
    (SOURCE) => {
        color_print::cstr!("<C!,k,s> FROM STR </>")
    };
    (UNKNOWN) => {
        color_print::cstr!("<M!,k,s> UNKNOWN </>")
    };
    (ID) => {
        color_print::cstr!("<M!,k,s> NO ID </>")
    };
    (NAME) => {
        color_print::cstr!("<M!,k,s> NO NAME </>")
    };
    (EXPRESSION) => {
        color_print::cstr!("<R!,k,s> INVALID EXPRESSION </>")
    };
}

/// Generate string literal `<INVALID `*XXX*`>`.
#[macro_export]
macro_rules! invalid_no_ansi {
    (VALUE) => {
        "<INVALID VALUE>"
    };
    (TYPE) => {
        "<INVALID TYPE>"
    };
    (OUTPUT) => {
        "<INVALID OUTPUT>"
    };
    (STACK) => {
        "<INVALID STACK>"
    };
    (REF) => {
        "<INVALID REF>"
    };
    (FILE) => {
        "<INVALID FILE>"
    };
    (RESULT) => {
        "<INVALID RESULT>"
    };
    (LINE) => {
        "<INVALID LINE>"
    };
    (SOURCE) => {
        "<FROM STR>"
    };
    (UNKNOWN) => {
        "<INVALID UNKNOWN>"
    };
    (ID) => {
        "<NO ID>"
    };
    (NAME) => {
        "<INVALID NAME>"
    };
    (EXPRESSION) => {
        "<INVALID EXPRESSION>"
    };
}

#[cfg(not(feature = "ansi-color"))]
macro_rules! invalid {
    ($x:literal) => {
        invalid_no_ansi!($x)
    };
}
