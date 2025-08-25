// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use crate::{eval::*, model::Models, syntax::*};

/// Trait for calling methods of values
pub trait CallMethod<T = Value> {
    /// Evaluate method call into a value (if possible)
    ///
    /// - `name`: Name of the method
    /// - `args`: Arguments for the method
    /// - `context`: Evaluation context
    fn call_method(
        &self,
        id: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<T>;
}

impl CallMethod for Array {
    fn call_method(
        &self,
        id: &QualifiedName,
        _: &ArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        match id.single_identifier().expect("Single id").id().as_str() {
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

impl CallMethod<Models> for Models {
    fn call_method(
        &self,
        name: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<Models> {
        // Try nest models for an operation: models.op()
        fn try_nest(
            symbol: &Symbol,
            name: &QualifiedName,
            args: &ArgumentValueList,
            context: &mut Context,
            models: &Models,
            op: Models,
        ) -> EvalResult<Models> {
            if op.is_operation() {
                if !models.contains_geometry() {
                    context.error(name, EvalError::OperationOnEmptyGeometry)?;
                }

                let models = op.nest(models);
                models.set_creator(symbol.clone(), SrcRef::merge(name, args));
                return Ok(models);
            } else {
                context.error(name, EvalError::NotAnOperation(name.clone()))?;
            }

            Ok(models.clone())
        }

        if let Some(symbol) = name.eval(context)? {
            Ok(match &symbol.borrow().def {
                SymbolDefinition::Workbench(workbench_definition) => {
                    let op = workbench_definition.call(args, context)?;
                    try_nest(&symbol, name, args, context, self, op)?
                }
                SymbolDefinition::Builtin(builtin) => match builtin.call(args, context)? {
                    Value::Models(models) => try_nest(&symbol, name, args, context, self, models)?,
                    value => panic!("Builtin call returned {value} but no models."),
                },
                def => {
                    context.error(
                        name,
                        EvalError::SymbolCannotBeCalled(name.clone(), Box::new(def.clone())),
                    )?;
                    Models::default()
                }
            })
        } else {
            Ok(Models::default())
        }
    }
}

impl CallMethod for Value {
    fn call_method(
        &self,
        id: &QualifiedName,
        args: &ArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        match self {
            Value::Integer(_) => eval_todo!(context, id, "call_method for Integer"),
            Value::Quantity(_) => eval_todo!(context, id, "call_method for Quantity"),
            Value::Bool(_) => eval_todo!(context, id, "call_method for Bool"),
            Value::String(_) => eval_todo!(context, id, "call_method for String"),
            Value::Tuple(_) => eval_todo!(context, id, "call_method for Tuple"),
            Value::Matrix(_) => eval_todo!(context, id, "call_method for Matrix"),
            Value::Array(list) => list.call_method(id, args, context),
            Value::Models(models) => Ok(Value::Models(models.call_method(id, args, context)?)),
            _ => {
                context.error(id, EvalError::UnknownMethod(id.clone()))?;
                Ok(Value::None)
            }
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
            &ArgumentValueList::default(),
            &mut Context::default(),
        )
        .expect("test error")
    {
        assert!(result);
    } else {
        panic!("Test failed");
    }
}
