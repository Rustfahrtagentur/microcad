// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Part definition syntax element

use crate::{src_ref::*, syntax::*};

/// Part definition
#[derive(Clone, Debug)]
pub struct PartDefinition {
    /// Part attributes.
    pub attribute_list: AttributeList,
    /// Part name.
    pub id: Identifier,
    /// Part parameters (implicit initialization).
    pub parameters: ParameterList,
    /// Part body
    pub body: Body,
    /// Part code reference
    pub src_ref: SrcRef,
}

impl PartDefinition {
    /// Return iterator over all initializers
    pub fn inits(&self) -> Inits {
        Inits::new(self)
    }
}

impl SrcReferrer for PartDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for PartDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "part {name}({parameters}) {body}",
            name = self.id,
            parameters = self.parameters,
            body = self.body
        )
    }
}

impl PrintSyntax for PartDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}PartDefinition '{}':", "", self.id)?;
        self.parameters.print_syntax(f, depth + 1)?;
        self.body.print_syntax(f, depth + 1)
    }
}
