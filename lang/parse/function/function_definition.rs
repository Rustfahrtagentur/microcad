// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*};

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

impl CallTrait for FunctionDefinition {
    type Output = Option<Value>;

    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Self::Output> {
        let arg_map = args.get_matching_arguments(context, &self.signature.parameters)?;

        let stack_frame = StackFrame::FunctionCall(context.top().symbol_table().clone());

        let mut result = None;
        context.scope(stack_frame, |context| {
            for (name, value) in arg_map.iter() {
                context.add(Symbol::Value(name.clone(), value.clone()));
            }

            for statement in self.body.0.iter() {
                if let Some(result_value) = statement.eval(context)? {
                    result = Some(result_value);
                    break;
                }
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl Parse for std::rc::Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.inner();
        let name = Identifier::parse(inner.next().unwrap())?;
        let signature = FunctionSignature::parse(inner.next().unwrap())?;
        let body = FunctionBody::parse(inner.next().unwrap())?;

        Ok(std::rc::Rc::new(FunctionDefinition {
            name,
            signature,
            body,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Eval for std::rc::Rc<FunctionDefinition> {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        context.add(self.clone().into());
        Ok(())
    }
}
