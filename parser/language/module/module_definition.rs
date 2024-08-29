use crate::{eval::*, language::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub parameters: Option<ParameterList>,
    pub body: ModuleBody,
}

impl ModuleDefinition {
    pub fn namespace(name: Identifier) -> Self {
        ModuleDefinition {
            attributes: Vec::new(),
            name,
            parameters: None,
            body: ModuleBody::new(),
        }
    }
}

impl Symbols for ModuleDefinition {
    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        self.body.find_symbols(name)
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleBody::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    attributes.push(Attribute::parse(pair)?.value);
                }
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value;
                }
                Rule::parameter_list => {
                    parameters = Some(ParameterList::parse(pair)?.value);
                }
                Rule::module_body => {
                    body = ModuleBody::parse(pair.clone())?.value;
                }
                rule => unreachable!("Unexpected module definition, got {:?}", rule),
            }
        }

        with_pair_ok!(
            ModuleDefinition {
                attributes,
                name,
                parameters,
                body,
            },
            pair
        )
    }
}
