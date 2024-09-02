//! Module definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;

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
    /// Create module definition
    pub fn namespace(name: Identifier) -> Self {
        ModuleDefinition {
            attributes: Vec::new(),
            name,
            parameters: None,
            body: ModuleBody::new(),
            src_ref: SrcRef(None),
        }
    }

    pub fn call(&self, _args: &CallArgumentList, _context: &mut Context) -> Result<tree::Node> {
        todo!()
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinition {
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.body.find_symbols(id)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add_symbol(symbol);
        self
    }
    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|symbol| {
            into.add_symbol(symbol.clone());
        });
    }
}

impl Parse for ModuleDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleBody::new();

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

        Ok(ModuleDefinition {
            attributes,
            name,
            parameters,
            body,
            src_ref: pair.into(),
        })
    }
}
