// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Module initialization definition
///
/// Example:
///
/// ```uCAD
/// module a {
///     init(b: length) {} // The init definition
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: Body,
    /// Source reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModuleInitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);

        Ok(ModuleInitDefinition {
            parameters: pair.find(Rule::parameter_list).unwrap_or_default(),
            body: pair.find(Rule::body).unwrap_or_default(),
            src_ref: pair.into(),
        })
    }
}

impl std::fmt::Display for ModuleInitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}

impl Syntax for ModuleInitDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition", "")?;
        writeln!(f, "{:depth$}  Parameters:", "")?;
        self.parameters.print_syntax(f, depth + 1)?;
        writeln!(f, "{:depth$}  Body:", "")?;
        self.body.print_syntax(f, depth + 1)
    }
}
