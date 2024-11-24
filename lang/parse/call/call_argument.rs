// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single call argument

use crate::{errors::*, eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// Call argument
#[derive(Clone, Debug)]
pub struct CallArgument {
    /// Name of the argument
    pub name: Option<Identifier>,
    /// Value of the argument
    pub value: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl CallArgument {
    /// Returns the name, if self.name is some. If self.name is None, try to extract the name from the expression
    pub fn derived_name(&self) -> Option<Identifier> {
        match &self.name {
            Some(name) => Some(name.clone()),
            None => self.value.single_identifier(),
        }
    }

    /// Evaluates the CallArgument and the parameter and return the matched value, if successful
    pub fn get_named_match(
        &self,
        context: &mut Context,
        param_value: &ParameterValue,
    ) -> Result<Value> {
        let arg_value = self.value.eval(context)?;
        if param_value.type_matches(&arg_value.ty()) {
            Ok(arg_value)
        } else {
            use crate::diag::PushDiag;
            context.error(
                self,
                anyhow::anyhow!(
                    "Type mismatch for parameter `{name}: Expected {expected}, got {got}.",
                    name = param_value.name,
                    expected = arg_value.ty(),
                    got = param_value.specified_type.as_ref().unwrap()
                ),
            )?;
            Ok(Value::Invalid)
        }
    }
}

impl SrcReferrer for CallArgument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Sym for CallArgument {
    fn id(&self) -> Option<microcad_core::Id> {
        if let Some(name) = &self.name {
            name.id()
        } else {
            None
        }
    }
}

impl OrdMapValue<Identifier> for CallArgument {
    fn key(&self) -> Option<Identifier> {
        self.name.clone()
    }
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.inner();
                let first = inner.next().unwrap();
                let second = inner.next().unwrap();

                Ok(CallArgument {
                    name: Some(Identifier::parse(first)?),
                    value: Expression::parse(second)?,
                    src_ref: pair.src_ref(),
                })
            }
            Rule::expression => Ok(CallArgument {
                name: None,
                value: Expression::parse(pair.clone())?,
                src_ref: pair.into(),
            }),
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl Eval for CallArgument {
    type Output = CallArgumentValue;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        Ok(CallArgumentValue::new(
            self.id(),
            self.value.eval(context)?,
            self.src_ref(),
        ))
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} = {}", name, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

#[test]
fn call_argument_match() {
    use crate::r#type::TypeAnnotation;

    // Make an argument: `name = i`
    let arg = |name: &str, i| -> CallArgument {
        CallArgument {
            name: if name.is_empty() {
                None
            } else {
                Some(name.into())
            },
            value: Expression::Literal(Literal::Integer(Refer::none(i))),
            src_ref: SrcRef(None),
        }
    };

    // Make a parameter value: `name: ty = i`
    let param = |name: &str, ty: Option<Type>, i: Option<i64>| -> ParameterValue {
        ParameterValue::new(
            name.into(),
            ty,
            i.map(|i| Value::Integer(Refer::none(i))),
            SrcRef(None),
        )
    };

    use crate::r#type::Type;

    let mut context = Context::default();
    // Check if argument `a = 10` matches parameter definition `a: int = 1`.
    match arg("a", 10).get_named_match(&mut context, &param("a", Some(Type::Integer), Some(1))) {
        Ok(Value::Integer(value)) => assert_eq!(value.value, 10, "Same value expected"),
        Ok(value) => panic!("Value mismatch, expected integer: {value}"),
        Err(err) => panic!("No match found: {err:?}"),
    }

    // Check if argument `a = 10` matches parameter definition `a: int`.
    match arg("a", 10).get_named_match(&mut context, &param("a", Some(Type::Integer), None)) {
        Ok(Value::Integer(value)) => assert_eq!(value.value, 10, "Same value expected"),
        Ok(value) => panic!("Value mismatch, expected integer: {value}"),
        Err(err) => panic!("No match found: {err:?}"),
    }

    match arg("a", 10).get_named_match(&mut context, &param("a", Some(Type::Angle), None)) {
        Ok(value) => print!("Value mismatch, expected integer: {value}"),
        Err(err) => panic!("No match found: {err:?}"),
    }
}
