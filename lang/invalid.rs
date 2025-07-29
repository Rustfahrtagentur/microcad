// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Macros to colorize invalid markers in logs if feature

#[cfg(feature = "ansi-color")]
mod macros {
    /// Generate string literal "INVALID XXX"
    #[macro_export]
    macro_rules! invalid {
        (VALUE) => {
            color_print::cstr!("<bg:r!><s> INVALID VALUE </s></bg:r!>")
        };
        (TYPE) => {
            color_print::cstr!("<bg:r!><s> INVALID TYPE </s></bg:r!>")
        };
        (OUTPUT) => {
            color_print::cstr!("<bg:r!><s> INVALID OUTPUT </s></bg:r!>")
        };
        (STACK) => {
            color_print::cstr!("<bg:w><s> EMPTY STACK </s></bg:w>")
        };
        (REF) => {
            color_print::cstr!("<bg:y!><s> NO REF </s></bg:y!>")
        };
        (FILE) => {
            color_print::cstr!("<bg:y!><s> NO FILE </s></bg:y!>")
        };
        (RESULT) => {
            color_print::cstr!("<bg:y!><s> NO RESULT </s></bg:y!>")
        };
        (LINE) => {
            color_print::cstr!("<bg:y!><s> NO LINE </s></bg:y!>")
        };
        (SOURCE) => {
            color_print::cstr!("<bg:c!><s> FROM STR </s></bg:c!>")
        };
        (UNKNOWN) => {
            color_print::cstr!("<bg:m!><s> UNKNOWN </s></bg:m!>")
        };
        (ID) => {
            color_print::cstr!("<bg:m!><s> NO ID </s></bg:m!>")
        };
        (NAME) => {
            color_print::cstr!("<bg:m!><s> NO NAME </s></bg:m!>")
        };
        (EXPRESSION) => {
            color_print::cstr!("<bg:r!><s> INVALID EXPRESSION </s></bg:r!>")
        };
    }

    /// Generate string literal "INVALID XXX"
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
            "<INVALID STR>"
        };
        (UNKNOWN) => {
            "<INVALID UNKNOWN>"
        };
        (ID) => {
            "<INVALID ID>"
        };
        (NAME) => {
            "<INVALID NAME>"
        };
        (EXPRESSION) => {
            color_print::cstr!("<INVALID EXPRESSION>")
        };
    }
}

#[cfg(not(feature = "ansi-color"))]
macro_rules! invalid {
    ($x:literal) => {
        invalid_no_ansi!($x)
    };
}
