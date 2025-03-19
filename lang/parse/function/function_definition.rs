// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Function definition
#[derive(Debug)]
pub struct FunctionDefinition {
    /// Name of the function
    pub name: Identifier,
    /// Function signature
    pub signature: FunctionSignature,
    /// Function body
    pub body: Body,
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
        body: Body,
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

impl Parse for std::rc::Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.inner();
        let name = Identifier::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let signature = FunctionSignature::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let body = Body::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;

        Ok(std::rc::Rc::new(FunctionDefinition {
            name,
            signature,
            body,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Syntax for FunctionDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}FunctionDefinition '{}'", "", self.name)?;
        writeln!(f, "{:depth$} Signature:", "")?;
        self.signature.print_syntax(f, depth + 2)?;
        writeln!(f, "{:depth$} Body:", "")?;
        self.body.print_syntax(f, depth + 2)
    }
}
