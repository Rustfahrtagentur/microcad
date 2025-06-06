// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in metadata.

use crate::{diag::*, eval::*, modeltree::*, syntax::*, ty::*, value::*};

/// Create built-in tag [`MetadataItem`].
pub fn tag(id: &Identifier) -> EvalResult<Option<MetadataItem>> {
    match id.id().as_str() {
        "aux" => Ok(Some(MetadataItem::Aux)),
        _ => Ok(None),
    }
}

/// Built-in export [`MetadataItem`]: `#export("filename.svg")`.
pub fn export(
    arguments: &CallArgumentValueList,
    context: &mut Context,
) -> EvalResult<Option<MetadataItem>> {
    // Convert the first argument to string and
    let arg_value = &arguments.get_single()?.value;
    match arg_value {
        Value::String(s) => Ok(Some(MetadataItem::Export(
            microcad_core::ExportSettings::with_filename(s.clone()),
        ))),
        value => {
            context.error(arguments, EvalError::InvalidArgumentType(value.ty()))?;
            Ok(None)
        }
    }
}

/// Built-in color [`MetadataItem`]: `#[color = "blue", fill_color = "#00FF00"]`.
pub fn color(
    id: &Identifier,
    expression: &Expression,
    context: &mut Context,
) -> EvalResult<Option<MetadataItem>> {
    match expression.eval(context)?.try_color() {
        Ok(color) => Ok(Some(MetadataItem::Color(id.clone(), color))),
        Err(err) => {
            context.error(expression, err)?;
            Ok(None)
        }
    }
}

/// A name value [`MetadataItem`], like `#[item_id = 2]`, `#[layer = "layer"]`.
pub fn name_id(
    id: &Identifier,
    expression: &Expression,
    context: &mut Context,
) -> EvalResult<Option<MetadataItem>> {
    let value = expression.eval(context)?;
    let id_str = id.id().as_str();

    match id_str {
        "layer" | "item_id" => match value {
            Value::Integer(_) | Value::String(_) => match id_str {
                "item_id" => Ok(Some(MetadataItem::ItemId(value))),
                "layer" => Ok(Some(MetadataItem::Layer(value))),
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
