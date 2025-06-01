// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export builtin attribute.

use crate::{diag::*, eval::*, objects::*, syntax::*, ty::*, value::*};

/// Built-in export attribute: `#export("filename.svg")`.
pub fn export(
    arguments: &CallArgumentValueList,
    context: &mut Context,
) -> EvalResult<Option<ObjectAttribute>> {
    // Convert the first argument to string and
    let arg_value = &arguments.get_single()?.value;
    match arg_value {
        Value::String(s) => Ok(Some(ObjectAttribute::Export(
            microcad_core::ExportSettings::with_filename(s.clone()),
        ))),
        value => {
            context.error(arguments, EvalError::InvalidArgumentType(value.ty()))?;
            Ok(None)
        }
    }
}

/// Built-in color attribute: `#[color = "blue", fill_color = "#00FF00"]`.
pub fn color(
    id: &Identifier,
    expression: &Expression,
    context: &mut Context,
) -> EvalResult<Option<ObjectAttribute>> {
    match expression.eval(context)?.try_color() {
        Ok(color) => Ok(Some(ObjectAttribute::Color(id.clone(), color))),
        Err(err) => {
            context.error(expression, err)?;
            Ok(None)
        }
    }
}

/// A name value attribute, like `#[part_id = 2]`, `#[layer = "layer"]`.
pub fn name_id(
    id: &Identifier,
    expression: &Expression,
    context: &mut Context,
) -> EvalResult<Option<ObjectAttribute>> {
    let value = expression.eval(context)?;
    let id_str = id.id().as_str();

    match id_str {
        "layer" | "part_id" => match value {
            Value::Integer(_) | Value::String(_) => match id_str {
                "part_id" => Ok(Some(ObjectAttribute::PartId(value))),
                "layer" => Ok(Some(ObjectAttribute::Layer(value))),
                _ => Ok(None),
            },
            _ => {
                context.error(
                    expression,
                    AttributeError::ExpectedType(
                        TypeList::new(vec![Type::Integer, Type::String]),
                        QualifiedName::from_id(id.clone()),
                        value.ty(),
                    ),
                )?;
                Ok(None)
            }
        },
        _ => Ok(None),
    }
}
