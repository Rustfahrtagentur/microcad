use crate::{parse::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    _parameters: ParameterList,
    _body: Vec<ModuleInitStatement>,
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);
        let mut parameters = ParameterList::default();
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?.value;
                }
                Rule::module_init_statement => {
                    body.push(ModuleInitStatement::parse(pair)?.value);
                }
                Rule::COMMENT => {}
                rule => unreachable!(
                    "expected parameter_list or module_init_statement. Instead found {rule:?}"
                ),
            }
        }

        with_pair_ok!(
            ModuleInitDefinition {
                _parameters: parameters,
                _body: body
            },
            pair
        )
    }
}
