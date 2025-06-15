// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::{eval::*, syntax::*};

/// Trait for calling methods of values
pub trait CallMethod {
    /// Evaluate method call into a value (if possible)
    ///
    /// - `name`: Name of the method
    /// - `args`: Arguments for the method
    /// - `context`: Evaluation context
    fn call_method(
        &self,
        id: &Identifier,
        args: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Value>;
}

impl CallMethod for List {
    fn call_method(
        &self,
        id: &Identifier,
        _: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        match id.id().as_str() {
            "count" => Ok(Value::Integer(self.len() as i64)),
            "all_equal" => {
                let is_equal = match self.first() {
                    Some(first) => self[1..].iter().all(|x| x == first),
                    None => true,
                };
                Ok(Value::Bool(is_equal))
            }
            "is_ascending" => {
                let is_ascending = self.as_slice().windows(2).all(|w| w[0] <= w[1]);
                Ok(Value::Bool(is_ascending))
            }
            "is_descending" => {
                let is_descending = self.as_slice().windows(2).all(|w| w[0] >= w[1]);
                Ok(Value::Bool(is_descending))
            }
            _ => {
                context.error(id, EvalError::UnknownMethod(id.clone()))?;
                Ok(Value::None)
            }
        }
    }
}

impl CallMethod for Value {
    fn call_method(
        &self,
        id: &Identifier,
        args: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        match &self {
            Value::None => todo!(),
            Value::Integer(_) => todo!(),
            Value::Quantity(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Color(_) => todo!(),
            Value::List(list) => list.call_method(id, args, context),
            Value::NamedTuple(_) => todo!(),
            Value::Tuple(_) => todo!(),
            Value::Matrix(_) => todo!(),
            Value::Nodes(_) => todo!(),
        }
    }
}

#[test]
fn call_list_method() {
    let list = List::new(
        ValueList::new(vec![
            Value::Quantity(Quantity::new(3.0, QuantityType::Scalar)),
            Value::Quantity(Quantity::new(3.0, QuantityType::Scalar)),
            Value::Quantity(Quantity::new(3.0, QuantityType::Scalar)),
        ]),
        crate::ty::Type::Quantity(QuantityType::Scalar),
    );

    if let Value::Bool(result) = list
        .call_method(
            &"all_equal".into(),
            &CallArgumentList::default(),
            &mut Context::default(),
        )
        .expect("test error")
    {
        assert!(result);
    } else {
        panic!("Test failed");
    }
}
