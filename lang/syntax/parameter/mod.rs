// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

mod parameter_list;

use crate::{diag::*, eval::*, ord_map::*, src_ref::*, syntax::*, ty::*, value::*};

pub use parameter_list::*;

/// A parameter of a parameter list.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    /// Name of the parameter
    pub id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Parameter {
    /// Create new parameter
    pub fn new(
        id: Identifier,
        specified_type: Option<TypeAnnotation>,
        default_value: Option<Expression>,
        src_ref: SrcRef,
    ) -> Self {
        assert!(!id.is_empty());
        Self {
            id,
            specified_type,
            default_value,
            src_ref,
        }
    }

    /// Create a new parameter without any SrcRef's
    pub fn no_ref(id: &str, ty: Type) -> Self {
        Self {
            id: Identifier::no_ref(id),
            specified_type: Some(TypeAnnotation(Refer::none(ty))),
            default_value: None,
            src_ref: SrcRef(None),
        }
    }

    /// Evaluate default value considering specified type
    ///
    /// If there is no default value, returns `Value::None` without raising an error.
    pub fn eval_default_value(&self, context: &mut Context) -> crate::eval::EvalResult<Value> {
        use crate::eval::Eval;

        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(default_value)) => {
                let value: Value = default_value.eval(context)?;
                if specified_type.ty() != value.ty() {
                    context.error(
                        self.src_ref.clone(),
                        EvalError::TypeMismatch {
                            id: self.id.clone(),
                            expected: specified_type.ty(),
                            found: value.ty(),
                        },
                    )?;
                    Ok(Value::None)
                } else {
                    Ok(value)
                }
            }
            (None, Some(default_value)) => Ok(default_value.eval(context)?),
            _ => Ok(Value::None),
        }
    }
}

impl SrcReferrer for Parameter {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.id.clone())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.id),
            (Some(t), None) => write!(f, "{}: {t}", self.id),
            (None, Some(v)) => write!(f, "{} = {v}", self.id),
            _ => Ok(()),
        }
    }
}

impl TreeDisplay for Parameter {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeIndent) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(default_value)) => writeln!(
                f,
                "{:depth$}Parameter: {}: {} = {}",
                "", self.id, specified_type, default_value
            ),
            (Some(specified_type), None) => {
                writeln!(f, "{:depth$}Parameter: {}: {}", "", self.id, specified_type)
            }
            (None, Some(default_value)) => {
                writeln!(f, "{:depth$}Parameter: {} = {}", "", self.id, default_value)
            }
            _ => unreachable!("impossible parameter declaration"),
        }
    }
}
