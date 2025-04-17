use crate::{parse::*, parser::*, rc_mut::*, syntax::*};

impl Parse for std::rc::Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut parameters = ParameterList::default();
        let mut body = Body::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::body => {
                    body = Body::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected rule for module definition, got {:?}", rule),
            }
        }

        Ok(Rc::new(ModuleDefinition {
            id: name,
            parameters,
            body,
            src_ref: pair.into(),
        }))
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
