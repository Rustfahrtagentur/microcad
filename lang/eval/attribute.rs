// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::BTreeMap;

use crate::{eval::*, model_tree::*, parameter, syntax::*};

use microcad_core::Color;
use thiserror::Error;

/// Error type for attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// Unknown attribute.
    #[error("Attribute not supported: {0}")]
    NotSupported(QualifiedName),

    /// The attribute expected a different type.
    #[error("Expected one of types `{0}` for attribute `{1}`, got `{2}`")]
    ExpectedType(TypeList, QualifiedName, Type),

    /// Attribute cannot be assigned to an expression.
    #[error("Cannot assign attribute to expression `{0}`")]
    CannotAssignToExpression(Box<Expression>),

    /// Warning when an attribute has already been set.
    #[error("The attribute is already set: {0} = {1}")]
    AttributeAlreadySet(Identifier, Value),
}

impl From<ArgumentMap> for NamedTuple {
    fn from(argument_map: ArgumentMap) -> Self {
        NamedTuple::new(
            argument_map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<BTreeMap<_, _>>(),
        )
    }
}

impl Attribute {
    /// Return a parameter list for an attribute
    fn parameter_list(id: &Identifier) -> Option<ParameterValueList> {
        match id.id().as_str() {
            "export" => Some(vec![parameter!(filename: String)].into()),
            _ => None,
        }
    }
}

impl Eval<Option<(Identifier, Value)>> for Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<(Identifier, Value)>> {
        match self {
            Attribute::NamedTuple(id, argument_list) => {
                if let Some(params) = Self::parameter_list(id) {
                    let args = ArgumentMap::find_match(&argument_list.eval(context)?, &params);
                    match args {
                        Ok(args) => {
                            return Ok(Some((id.clone(), Value::NamedTuple(args.into()))));
                        }
                        Err(err) => {
                            context.warning(self, err)?;
                        }
                    }
                }
            }
            Attribute::NameValue(id, expression) => {
                return Ok(Some((id.clone(), expression.eval(context)?)));
            }
        }

        Ok(None)
    }
}

impl AttributeList {
    /// Default parameters for name-value attributes.
    ///
    /// Only name-value attributes that are in this list are allowed.
    fn default_parameter_list() -> ParameterValueList {
        vec![
            parameter!(layer: String = "default".into()),
            parameter!(color: Color = Color::default()),
        ]
        .into()
    }

    /// Evaluate name value attributes and check if they match with the default parameter list.
    fn eval_name_value_attributes(
        &self,
        context: &mut Context,
    ) -> EvalResult<Vec<(Identifier, Value)>> {
        let attributes: Result<Vec<_>, _> = self
            .iter()
            .filter_map(|attr| {
                if matches!(attr, Attribute::NameValue(_, _)) {
                    attr.eval(context).transpose()
                } else {
                    None
                }
            })
            .collect();
        let attributes = attributes?;

        // Build a `ArgumentValueList` from the attributes...
        let mut args = ArgumentValueList::default();
        for (id, value) in attributes {
            args.try_push(ArgumentValue::new(Some(id), value, SrcRef(None)))
                .expect("Argument");
        }

        // ... and check if it matches with the default parameter list.
        let args = ArgumentMap::find_match(&args, &Self::default_parameter_list());
        match args {
            Ok(args) => Ok(args.iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
            Err(err) => {
                context.warning(self, err)?;
                Ok(vec![])
            }
        }
    }

    /// Evaluate named tuple attributes.
    fn eval_named_tuple_attributes(
        &self,
        context: &mut Context,
    ) -> EvalResult<Vec<(Identifier, Value)>> {
        self.iter()
            .filter_map(|attr| {
                if matches!(attr, Attribute::NamedTuple(_, _)) {
                    attr.eval(context).transpose()
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Eval<Metadata> for AttributeList {
    fn eval(&self, context: &mut Context) -> EvalResult<Metadata> {
        // Split attribute list into named tuple and name value attributes
        let name_value_attributes = self.eval_name_value_attributes(context)?;
        let named_tuple_attributes = self.eval_named_tuple_attributes(context)?;

        let mut map = BTreeMap::new();
        for (id, value) in name_value_attributes
            .iter()
            .chain(named_tuple_attributes.iter())
        {
            if map.contains_key(id) {
                context.warning(
                    self,
                    AttributeError::AttributeAlreadySet(id.clone(), value.clone()),
                )?;
            }
            map.insert(id.clone(), value.clone());
        }

        Ok(Metadata::new(map))
    }
}
