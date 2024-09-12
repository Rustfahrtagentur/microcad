// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module attributes
    pub attributes: Vec<Attribute>,
    /// Module name
    pub name: Identifier,
    /// Module Parameters
    pub parameters: Option<ParameterList>,
    /// Module body
    pub body: ModuleBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Create new module definition
    pub fn new(name: Identifier) -> Self {
        ModuleDefinition {
            attributes: Vec::new(),
            name,
            parameters: None,
            body: ModuleBody::default(),
            src_ref: SrcRef(None),
        }
    }
}

impl CallTrait for ModuleDefinition {
    fn call(&self, _args: &CallArgumentList, _context: &mut Context) -> Result<Option<Value>> {
        todo!()
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinition {
    fn fetch(&self, id: &Id) -> Vec<&Symbol> {
        self.body.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add(symbol);
        self
    }
    fn copy<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|symbol| {
            into.add(symbol.clone());
        });
    }
}

impl Parse for std::rc::Rc<ModuleDefinition> {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleBody::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    attributes.push(Attribute::parse(pair)?);
                }
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::parameter_list => {
                    parameters = Some(ParameterList::parse(pair)?);
                }
                Rule::module_body => {
                    body = ModuleBody::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected module definition, got {:?}", rule),
            }
        }

        Ok(std::rc::Rc::new(ModuleDefinition {
            attributes,
            name,
            parameters,
            body,
            src_ref: pair.into(),
        }))
    }
}
