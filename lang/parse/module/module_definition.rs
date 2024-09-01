use crate::{eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub parameters: Option<ParameterList>,
    pub body: ModuleBody,
    src_ref: SrcRef,
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl ModuleDefinition {
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
