//! For statement parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// For statement
#[derive(Clone, Debug)]
pub struct ForStatement {
    /// Initial assignment
    loop_var: Assignment,
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
        Parser::ensure_rule(&pair, Rule::module_for_statement);

        let src_ref = pair.clone().into();
        let mut pairs = pair.into_inner();

        Ok(ForStatement {
            loop_var: Assignment::parse(pairs.next().unwrap())?,
            body: ModuleBody::parse(pairs.next().unwrap())?,
            src_ref,
        })
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}
