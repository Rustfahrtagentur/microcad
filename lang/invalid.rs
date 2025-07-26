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
            color_print::cstr!("<bg:m!> UNKNOWN </bg:m!>")
        };
        (ID) => {
            color_print::cstr!("<bg:m!> NO ID </bg:m!>")
        };
        (NAME) => {
            color_print::cstr!("<bg:m!> NO NAME </bg:m!>")
        };
    }
}

#[cfg(not(feature = "ansi-color"))]
mod macros {
    /// Generate string literal "INVALID XXX"
    #[macro_export]
    macro_rules! invalid {
        (VALUE) => {
            "VALUE"
        };
        (TYPE) => {
            "TYPE"
        };
        (OUTPUT) => {
            "OUTPUT"
        };
        (STACK) => {
            "STACK"
        };
        (REF) => {
            "REF"
        };
        (FILE) => {
            "FILE"
        };
        (RESULT) => {
            "RESULT"
        };
        (LINE) => {
            "LINE"
        };
        (SOURCE) => {
            "STR"
        };
        (UNKNOWN) => {
            "UNKNOWN"
        };
        (ID) => {
            "ID"
        };
        (NAME) => {
            "NAME"
        };
    }
}
