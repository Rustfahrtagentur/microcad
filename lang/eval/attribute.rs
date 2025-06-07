// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*, syntax::*};

use thiserror::Error;

/// Error type for attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// Unknown attribute.
    #[error("Unknown attribute: {0}")]
    Unknown(QualifiedName),

    /// The attribute expected a different type.
    #[error("Expected one of types `{0}` for attribute `{1}`, got {2}")]
    ExpectedType(TypeList, QualifiedName, Type),

    /// Attribute cannot be assigned to an expression.
    #[error("Cannot assign attribute to expression `{0}`")]
    CannotAssignToExpression(Box<Expression>),
}

impl Eval<Option<MetadataItem>> for Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<MetadataItem>> {
        let qualified_name = self.qualified_name();
        if let Some(id) = qualified_name.single_identifier() {
            let str = id.id().as_str();

            use crate::builtin::*;

            match self {
                Attribute::Tag(tag) => {
                    if let Some(id) = tag.single_identifier() {
                        return attributes::tag(id);
                    }
                }
                Attribute::Call(call) => {
                    let arguments = call.argument_list.eval(context)?;

                    if str == "export" {
                        return attributes::export(&arguments, context);
                    }
                }
                Attribute::NameValue(_, expression) => match str {
                    "color" | "stroke_color" | "fill_color" => {
                        return attributes::color(id, expression, context);
                    }
                    "part" | "layer" => {
                        return attributes::name_id(id, expression, context);
                    }
                    _ => {}
                },
            }
        }

        context.warning(self, AttributeError::Unknown(qualified_name.clone()))?;
        Ok(None)
    }
}

impl Eval<Metadata> for AttributeList {
    fn eval(&self, context: &mut Context) -> EvalResult<Metadata> {
        let mut metadata = Metadata::default();
        for attribute in self.iter() {
            if let Some(item) = attribute.eval(context)? {
                metadata.push(item)
            }
        }

        Ok(metadata)
    }
}
