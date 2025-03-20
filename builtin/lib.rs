// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

use microcad_lang::{src_ref::SrcReferrer, *};

/// Build the standard module
pub fn builtin_module() -> RcMut<SymbolNode> {
    let builtin_namespace = NamespaceDefinition::new("__builtin".into());
    let mut builtin_symbol = SymbolNode::new(SymbolDefinition::Namespace(builtin_namespace), None);

    let assert_valid = BuiltinFunction::new("assert_valid".into(), &|args, context| {
        println!("assert valid called");
        for arg in args.iter() {
            if let Expression::Nested(nested) = &arg.value {
                if let Some(qualified_name) = nested.single_qualified_name() {
                    println!("{}", context.current_node().borrow());

                    if context.fetch_symbol(&qualified_name).is_err()
                        && context
                            .fetch_local(&qualified_name.clone().try_into()?)
                            .is_err()
                    {
                        panic!(
                            "Symbol {} invalid in {}",
                            qualified_name,
                            context.ref_str(&qualified_name)
                        );
                    }
                }
            }
        }
        Ok(Value::Invalid)
    });
    let assert_invalid = BuiltinFunction::new("assert_invalid".into(), &|args, context| {
        println!("assert invalid called");
        for arg in args.iter() {
            if let Expression::Nested(nested) = &arg.value {
                if let Some(qualified_name) = nested.single_qualified_name() {
                    if context.fetch_symbol(&qualified_name).is_ok() {
                        panic!(
                            "Symbol {} valid in {}",
                            qualified_name,
                            context.ref_str(&qualified_name)
                        );
                    }
                }
            }
        }
        Ok(Value::Invalid)
    });

    SymbolNode::insert_child(
        &mut builtin_symbol,
        SymbolNode::new(SymbolDefinition::BuiltinFunction(assert_valid), None),
    );
    SymbolNode::insert_child(
        &mut builtin_symbol,
        SymbolNode::new(SymbolDefinition::BuiltinFunction(assert_invalid), None),
    );

    builtin_symbol
}
