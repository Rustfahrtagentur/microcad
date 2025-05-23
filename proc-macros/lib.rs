// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Helper macros to instrument renderable builtin symbols

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Create test of microcad code
///
/// Example:
/// ```
/// #[microcad]
/// fn my_test() {
///   std::print("printed from microcad";
/// }
/// ```
#[proc_macro_attribute]
pub fn microcad(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    // Extract the content inside the curly braces
    let block_content = quote! { #fn_block }
        .to_string()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .to_string();

    // Generate the new function with the test! macro
    let output = quote! {
        #[test]
        fn #fn_name() {
            let source = match crate::SourceFile::load_from_str(#block_content) {
                Err(err) => panic!("Parse error:\n{err}"),
                Ok(source) => source
            };
            let symbols = source.resolve(None);
            let mut context = crate::Context::new(
                symbols.clone(),
                microcad_builtin::builtin_namespace(),
                &["../lib".into()],
                Box::new(Stdout),
            );
            match context.eval() {
                Err(err) => panic!("Evaluation error(s):\n{context}"),
                Ok(_) => ()
            }
        }
    };

    output.into()
}
