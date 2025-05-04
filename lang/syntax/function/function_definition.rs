// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};

/// Function definition
#[derive(Debug)]
pub struct FunctionDefinition {
    /// Name of the function
    pub id: Identifier,
    /// Function signature
    pub signature: FunctionSignature,
    /// Function body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for FunctionDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionDefinition {
    /// Create new function definition
    pub fn new(id: Identifier, signature: FunctionSignature, body: Body, src_ref: SrcRef) -> Self {
        Self {
            id,
            signature,
            body,
            src_ref,
        }
    }

    /// Resolve into SymbolNode
    pub fn resolve(self: &Rc<Self>, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Function(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }
}

impl PrintSyntax for FunctionDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}FunctionDefinition '{}':", "", self.id)?;
        writeln!(f, "{:depth$} Signature:", "")?;
        self.signature.print_syntax(f, depth + 2)?;
        writeln!(f, "{:depth$} Body:", "")?;
        self.body.print_syntax(f, depth + 2)
    }
}
