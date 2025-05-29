// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*, syntax::*};

use thiserror::Error;

#[derive(Debug, Error)]

pub enum AttributeError {
    #[error("Unknown attribute: {0}")]
    Unknown(QualifiedName),

    #[error("Type `{0}` expected for attribute `{1}`")]
    TypeExpected(Type, QualifiedName),
}

impl Attribute {
    /// Evaluate [Attribute] to [ObjectAttribute].
    ///
    /// This functions contains all built-in attributes for objects.
    pub fn eval_to_object_attribute(
        &self,
        context: &mut Context,
    ) -> EvalResult<Option<ObjectAttribute>> {
        let qualified_name = self.qualified_name();
        if let Some(id) = qualified_name.single_identifier() {
            let str = id.id().as_str();

            match self {
                Attribute::Call(call) => {
                    let arguments = call.argument_list.eval(context)?;

                    if str == "export" {
                        return crate::builtin::attributes::export(&arguments, context);
                    }
                }
                Attribute::NameValue(_, expression) => match str {
                    "color" | "stroke_color" | "fill_color" => {
                        return crate::builtin::attributes::color(id, expression, context);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        context.warning(self, AttributeError::Unknown(qualified_name.clone()).into())?;
        Ok(None)
    }
}

impl AttributeList {
    /// Evaluate the attribute list into [ObjectAttributes].
    pub fn eval_to_object_attributes(&self, context: &mut Context) -> EvalResult<ObjectAttributes> {
        let mut object_attributes = ObjectAttributes::default();

        for attribute in self.iter() {
            if let Some(attribute) = attribute.eval_to_object_attribute(context)? {
                object_attributes.push(attribute)
            }
        }

        Ok(object_attributes)
    }
}
