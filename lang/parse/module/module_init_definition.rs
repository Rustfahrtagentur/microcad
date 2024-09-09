//! Module initialization definition parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Module initialization definition
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    _parameters: ParameterList,
    _body: Vec<ModuleInitStatement>,
    src_ref: SrcRef,
}

impl SrcReferrer for ModuleInitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);
        let mut parameters = ParameterList::default();
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::module_init_statement => {
                    body.push(ModuleInitStatement::parse(pair)?);
                }
                Rule::COMMENT => {}
                rule => unreachable!(
                    "expected parameter_list or module_init_statement. Instead found {rule:?}"
                ),
            }
        }

        Ok(ModuleInitDefinition {
            _parameters: parameters,
            _body: body,
            src_ref: pair.into(),
        })
    }
}
