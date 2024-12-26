// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};

/// Function definition
#[derive(Debug)]
pub struct FunctionDefinition {
    /// NAme of the function
    pub name: Identifier,
    /// Function signature
    pub signature: FunctionSignature,
    /// Function body
    pub body: FunctionBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for FunctionDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionDefinition {
    /// Create new function definition
    pub fn new(
        name: Identifier,
        signature: FunctionSignature,
        body: FunctionBody,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            name,
            signature,
            body,
            src_ref,
        }
    }
}

impl CallTrait for std::rc::Rc<FunctionDefinition> {
    type Output = Value;

    fn call(&self, args: &CallArgumentList, context: &mut Context) -> EvalResult<Self::Output> {
        let stack_frame = StackFrame::function(context, self.clone());

        context.scope(stack_frame, |context| {
            let arg_map = args.get_matching_arguments(context, &self.signature.parameters)?;

            for (name, value) in arg_map.iter() {
                context.add(Symbol::Value(name.clone(), value.clone()));
            }

            for statement in self.body.0.iter() {
                if let Some(result_value) = statement.eval(context)? {
                    return Ok(result_value);
                }
            }
            Ok(Value::Invalid)
        })
    }
}

impl Parse for std::rc::Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.inner();
        let name = Identifier::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let signature = FunctionSignature::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let body = FunctionBody::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;

        Ok(std::rc::Rc::new(FunctionDefinition {
            name,
            signature,
            body,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Eval for std::rc::Rc<FunctionDefinition> {
    type Output = Symbol;

    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output> {
        context.add(self.clone().into());
        Ok(Symbol::Function(self.clone()))
    }
}
