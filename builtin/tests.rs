// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{ContextBuilder, NamespaceBuilder};
use log::info;
use microcad_lang::{eval::*, sym::*};

#[test]
fn context_namespace() {
    use microcad_lang::src_ref::*;

    let mut context = EvalContext::default();

    let module = NamespaceBuilder::new("math")
        .add(Symbol::Value(
            "pi".into(),
            Value::Scalar(Refer::none(std::f64::consts::PI)),
        ))
        .build();

    context.add(module.into());

    let symbols = context
        .fetch_symbols_by_qualified_name(&"math::pi".into())
        .expect("test error");
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].id().expect("test error"), "pi");
}

#[test]
fn test_assert() {
    use microcad_lang::parse::source_file::SourceFile;

    let source_file = match SourceFile::load_from_str(
        r#"
            __builtin::assert(__builtin::math::abs(-1.0) == 1.0);
        "#,
    ) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = ContextBuilder::new(source_file)
        .with_builtin()
        .expect("builtin error")
        .build();

    match context.eval() {
        Ok(_) => {
            info!("Our assertion was successful as expected");
        }
        Err(err) => panic!("{err}"),
    }
}
