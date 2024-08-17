use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::{expression::*, function::DefinitionParameter, identifier::*, lang_type::Ty, value::*};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct CallArgument {
    name: Option<Identifier>,
    value: Expression,
}

impl CallArgument {
    pub fn name(&self) -> Option<&Identifier> {
        self.name.as_ref()
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Parse for CallArgument {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.clone().into_inner();
                let first = inner.next().unwrap();
                let second = inner.next().unwrap();

                with_pair_ok!(
                    CallArgument {
                        name: Some(Identifier::parse(first)?.value().clone()),
                        value: Expression::parse(second)?.value().clone(),
                    },
                    pair
                )
            }
            Rule::expression => {
                with_pair_ok!(
                    CallArgument {
                        name: None,
                        value: Expression::parse(pair.clone())?.value().clone(),
                    },
                    pair
                )
            }
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} = {}", name, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CallArgumentList {
    arguments: Vec<CallArgument>,
    named: HashMap<Identifier, usize>,
}

impl CallArgumentList {
    pub fn push(&mut self, arg: CallArgument) {
        self.arguments.push(arg.clone());
        if let Some(name) = arg.name() {
            self.named.insert(name.clone(), self.arguments.len() - 1);
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<&CallArgument> {
        self.named.get(name).map(|index| &self.arguments[*index])
    }

    pub fn match_definition(
        &self,
        parameters: &Vec<DefinitionParameter>,
        context: &mut Context,
    ) -> Result<ArgumentMap, Error> {
        self._match_definition(parameters, context, true)
    }

    pub fn match_definition_no_type_check(
        &self,
        parameters: &Vec<DefinitionParameter>,
        context: &mut Context,
    ) -> Result<ArgumentMap, Error> {
        self._match_definition(parameters, context, false)
    }

    fn _match_definition(
        &self,
        parameters: &Vec<DefinitionParameter>,
        context: &mut Context,
        check_types: bool,
    ) -> Result<ArgumentMap, Error> {
        let mut arg_map = ArgumentMap::new();

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        for name in self.named.keys() {
            if !parameters.iter().any(|p| p.name() == name) {
                return Err(Error::UnexpectedArgument(name.clone()));
            }
        }

        let mut eval_params = Vec::new();
        for param in parameters {
            if check_types {
                let (default_value, ty) = param.eval(context)?;
                eval_params.push((param.name(), default_value, Some(ty)));
            } else {
                eval_params.push((
                    param.name(),
                    match &param.default_value() {
                        Some(default) => Some(default.eval(context)?),
                        None => None,
                    },
                    None,
                ));
            }
        }

        // Check for matching named arguments
        for (param_name, param_default_value, param_ty) in &eval_params {
            match self.get(param_name) {
                Some(arg) => {
                    let value = arg.value.eval(context)?;
                    if !check_types || value.ty() == *param_ty.as_ref().unwrap() {
                        arg_map.insert((*param_name).clone(), value);
                    } // @todo Throw error on else?
                }
                None => {
                    if let Some(default) = param_default_value {
                        arg_map.insert((*param_name).clone(), default.clone());
                    }
                }
            }
        }

        // Check for matching positional arguments
        let mut positional_index = 0;
        for arg in &self.arguments {
            if arg.name.is_none() {
                let (param_name, _, param_ty) = &eval_params[positional_index];
                if !arg_map.contains_key(param_name)
                    && (!check_types
                        || *param_ty.as_ref().unwrap() == arg.value.eval(context)?.ty())
                {
                    arg_map.insert((*param_name).clone(), arg.value.eval(context)?);
                    positional_index += 1;
                }
            }
        }

        // Finally, we need to check if all arguments have been matched
        let mut missing_args = IdentifierList::new();
        for (param_name, _, _) in &eval_params {
            if !arg_map.contains_key(param_name) {
                missing_args.push((*param_name).clone()).unwrap();
            }
        }
        if !missing_args.is_empty() {
            return Err(Error::MissingArguments(missing_args));
        }

        Ok(arg_map)
    }
}

impl Deref for CallArgumentList {
    type Target = Vec<CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.arguments
    }
}

pub struct ArgumentMap(HashMap<Identifier, Value>);

impl ArgumentMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Default for ArgumentMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ArgumentMap {
    type Target = HashMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut call_argument_list = CallArgumentList::default();

        match pair.clone().as_rule() {
            Rule::call_argument_list => {
                for pair in pair.clone().into_inner() {
                    let call = CallArgument::parse(pair.clone())?.value().clone();
                    match call.name {
                        Some(ident) => {
                            call_argument_list.push(CallArgument {
                                name: Some(ident),
                                value: call.value,
                            });
                        }
                        None => {
                            call_argument_list.push(CallArgument {
                                name: None,
                                value: call.value,
                            });
                        }
                    }
                }

                with_pair_ok!(call_argument_list, pair)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        println!("{inner:?}");

        with_pair_ok!(
            MethodCall {
                name: Identifier::parse(inner.next().unwrap())?.value().clone(),
                argument_list: if let Some(pair) = inner.next() {
                    CallArgumentList::parse(pair)?.value().clone()
                } else {
                    CallArgumentList::default()
                },
            },
            pair
        )
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Call {
    #[allow(dead_code)]
    pub name: QualifiedName,
    #[allow(dead_code)]
    pub argument_list: CallArgumentList,
}

impl Parse for Call {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();

        with_pair_ok!(
            Call {
                name: QualifiedName::parse(first)?.value().clone(),
                argument_list: match inner.next() {
                    Some(pair) => CallArgumentList::parse(pair)?.value().clone(),
                    None => CallArgumentList::default(),
                }
            },
            pair
        )
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}

impl Call {}

impl Eval for Call {
    type Output = Option<Value>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let symbols = self.name.eval(context)?;

        for symbol in symbols {
            match symbol {
                Symbol::Function(f) => {
                    if let Ok(value) = f.call(&self.argument_list, context) {
                        return Ok(value);
                    }
                }
                Symbol::BuiltinFunction(f) => {
                    if let Ok(value) = f.call(&self.argument_list, context) {
                        return Ok(value);
                    }
                }
                Symbol::BuiltinModule(m) => {
                    if let Ok(value) = m.call(&self.argument_list, context) {
                        return Ok(Some(Value::Node(value)));
                    }
                }
                _ => unimplemented!("Call::eval for symbol"),
            }
        }

        Err(Error::SymbolNotFound(self.name.clone()))
    }
}

#[test]
fn call() {
    use pest::Parser as _;
    let pair = Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
        .unwrap()
        .next()
        .unwrap();

    let call = Call::parse(pair).unwrap();

    assert_eq!(call.name, QualifiedName::from("foo"));
    assert_eq!(call.argument_list.len(), 4);

    // Count named arguments
    let named = call
        .argument_list
        .iter()
        .filter(|arg| arg.name.is_some())
        .count();
    assert_eq!(named, 2);
}
