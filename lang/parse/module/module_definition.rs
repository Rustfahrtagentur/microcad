// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub name: Identifier,
    /// Module parameters (implicit initialization)
    pub parameters: Option<ParameterList>,
    /// Module body
    pub body: Body,
    /// Source code reference
    src_ref: SrcRef,
}

/// Match of an initializer
///
/// This enum represents a match of an initializer containing the initializer itself and the argument map
enum InitializerMatch {
    /// Match of an implicit initializer
    Implicit(std::rc::Rc<ModuleInitDefinition>, MultiArgumentMap),

    /// Match of an explicit initializer
    Explicit(std::rc::Rc<ModuleInitDefinition>, MultiArgumentMap),
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for std::rc::Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = Body::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::parameter_list => {
                    parameters = Some(ParameterList::parse(pair)?);
                }
                Rule::module_body => {
                    body = Body::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected rule for module definition, got {:?}", rule),
            }
        }

        Ok(std::rc::Rc::new(ModuleDefinition {
            name,
            parameters,
            body,
            src_ref: pair.into(),
        }))
    }
}
