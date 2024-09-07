//! For statement parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// For statement
#[derive(Clone, Debug)]
pub struct ForStatement {
    /// Loop variable
    loop_var: Identifier,
    /// Loop expression
    loop_expr: Expression,
    /// For loop body
    body: ModuleBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for ForStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ForStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::for_statement);

        let mut pairs = pair.clone().into_inner();

        Ok(ForStatement {
            loop_var: Identifier::parse(pairs.next().unwrap())?,
            loop_expr: Expression::parse(pairs.next().unwrap())?,
            body: ModuleBody::parse(pairs.next().unwrap())?,
            src_ref: pair.into(),
        })
    }
}

impl Eval for ForStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
        match self.loop_expr.eval(context)? {
            Value::List(list) => {
                for value in list.iter() {
                    context.push();
                    context.add_symbol(Symbol::Value(self.loop_var.id().unwrap(), value.clone()));
                    self.body.eval(context)?;
                    context.pop();
                }
            }
            value => {
                use crate::diagnostics::AddDiagnostic;
                context.error(self, format!("Expected list, got {}", value.ty()));
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}
