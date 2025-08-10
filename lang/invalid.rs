// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Macros to colorize invalid markers in logs if feature

#[cfg(feature = "ansi-color")]
mod macros {
    /// Generate string literal "INVALID XXX"
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
            "<FROM STR>"
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
