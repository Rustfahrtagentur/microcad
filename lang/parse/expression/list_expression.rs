// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List expression

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// List expression (expression list maybe with common unit)
#[derive(Default, Clone, Debug)]
pub struct ListExpression {
    list: ExpressionList,
    unit: Option<Unit>,
    src_ref: SrcRef,
}

impl std::ops::Deref for ListExpression {
    type Target = ExpressionList;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl std::ops::DerefMut for ListExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl SrcReferrer for ListExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        Ok(Self {
            list: ExpressionList::parse(inner.next().unwrap())?,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl std::fmt::Display for ListExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]{}",
            self.list
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            if let Some(unit) = self.unit {
                unit.to_string()
            } else {
                String::new()
            }
        )?;

        Ok(())
    }
}

impl Eval for ListExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
        let mut value_list = ValueList::new(Vec::new(), self.src_ref());
        for expr in self.list.clone() {
            value_list.push(expr.eval(context)?);
        }
        if let Some(unit) = self.unit {
            value_list.add_unit_to_unitless(unit)?;
        }

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::List(List::new(
                value_list,
                common_type,
                self.src_ref(),
            ))),
            None => Err(EvalError::ListElementsDifferentTypes),
        }
    }
}
