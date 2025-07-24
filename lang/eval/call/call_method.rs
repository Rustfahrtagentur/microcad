// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

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
        args: &ArgumentList,
        context: &mut Context,
    ) -> EvalResult<Value>;
}

impl CallMethod for Array {
    fn call_method(
        &self,
        id: &Identifier,
        _: &ArgumentList,
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
        args: &ArgumentList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        match &self {
            Value::None => unreachable!("None value cannot be called"),
            Value::Integer(_) => eval_todo!(context, args, "call_method for Integer"),
            Value::Quantity(_) => eval_todo!(context, args, "call_method for Quantity"),
            Value::Bool(_) => eval_todo!(context, args, "call_method for Bool"),
            Value::String(_) => eval_todo!(context, args, "call_method for String"),
            Value::Array(list) => list.call_method(id, args, context),
            Value::Tuple(_) => eval_todo!(context, args, "call_method for Tuple"),
            Value::Matrix(_) => eval_todo!(context, args, "call_method for Matrix"),
            Value::Models(_) => eval_todo!(context, args, "call_method for Models"),
            Value::Return(_) => unreachable!("Return value cannot be called"),
        }
    }
}

#[test]
fn call_list_method() {
    let list = Array::new(
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
            &ArgumentList::default(),
            &mut Context::default(),
        )
        .expect("test error")
    {
        assert!(result);
    } else {
        panic!("Test failed");
    }
}
