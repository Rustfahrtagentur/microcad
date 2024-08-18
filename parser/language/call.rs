use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::{expression::*, identifier::*, lang_type::Ty, parameter::*, value::*};
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

#[derive(Clone, Debug)]
pub struct CallArgumentValue {
    name: Option<Identifier>,
    value: Value,
}

impl Eval for CallArgument {
    type Output = CallArgumentValue;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        Ok(CallArgumentValue {
            name: self.name.clone(),
            value: self.value.eval(context)?,
        })
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
}

impl Deref for CallArgumentList {
    type Target = Vec<CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.arguments
    }
}

#[derive(Clone, Debug)]
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
pub struct CallArgumentValueList {
    arguments: Vec<CallArgumentValue>,
    named: HashMap<Identifier, usize>,
}

impl CallArgumentValueList {
    pub fn get(&self, name: &Identifier) -> Option<&CallArgumentValue> {
        self.named.get(name).map(|index| &self.arguments[*index])
    }

    pub fn push(&mut self, arg: CallArgumentValue) {
        self.arguments.push(arg.clone());
        if let Some(name) = arg.name {
            self.named.insert(name.clone(), self.arguments.len() - 1);
        }
    }

    pub fn match_definition(
        &self,
        parameter_values: &ParameterValueList,
    ) -> Result<ArgumentMap, Error> {
        let mut arg_map = ArgumentMap::new();

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        for name in self.named.keys() {
            if parameter_values.get(name).is_none() {
                return Err(Error::UnexpectedArgument(name.clone()));
            }
        }

        // Check for matching named arguments
        // Iterate over defined parameters and check if the call arguments contains an argument with the same as the parameter
        for parameter_value in parameter_values.iter() {
            let ParameterValue {
                name,
                default_value,
                ..
            } = parameter_value;

            match self.get(name) {
                // We have a matching argument with the same name as the parameter.
                Some(arg) => {
                    // Now we need to check if the argument type matches the parameter type
                    if parameter_value.type_check(&arg.value.ty())? {
                        arg_map.insert(name.clone(), arg.value.clone());
                    }
                }
                // No matching argument found, check if a default value is defined
                None => {
                    // If we have a default value, we can use it
                    if let Some(default) = default_value {
                        arg_map.insert(name.clone(), default.clone());
                    }
                }
            }
        }

        // Check for matching positional arguments
        // @todo: All check for tuple arguments and if the tuple fields match the parameters
        let mut positional_index = 0;
        for arg in &self.arguments {
            if arg.name.is_none() {
                let ParameterValue { name, .. } = &parameter_values[positional_index];
                if !arg_map.contains_key(name)
                    && (parameter_values[positional_index].type_check(&arg.value.ty())?)
                {
                    arg_map.insert((*name).clone(), arg.value.clone());
                    positional_index += 1;
                }
            }
        }

        // Finally, we need to check if all arguments have been matched
        let mut missing_args = IdentifierList::new();
        for ParameterValue { name, .. } in parameter_values.iter() {
            if !arg_map.contains_key(name) {
                missing_args.push(name.clone()).unwrap();
            }
        }
        if !missing_args.is_empty() {
            return Err(Error::MissingArguments(missing_args));
        }

        Ok(arg_map)
    }
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let mut call_argument_list = CallArgumentValueList::default();

        for arg in &self.arguments {
            call_argument_list.push(arg.eval(context)?);
        }

        Ok(call_argument_list)
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
        let mut non_matching_symbols = Vec::new();
        for symbol in &symbols {
            match symbol {
                Symbol::Function(f) => {
                    if let Ok(value) = f.call(&self.argument_list, context) {
                        return Ok(value);
                    } else {
                        non_matching_symbols.push(symbol.clone());
                    }
                }
                Symbol::BuiltinFunction(f) => {
                    if let Ok(value) = f.call(&self.argument_list, context) {
                        return Ok(value);
                    } else {
                        non_matching_symbols.push(symbol.clone());
                    }
                }
                Symbol::BuiltinModule(m) => {
                    if let Ok(value) = m.call(&self.argument_list, context) {
                        return Ok(Some(Value::Node(value)));
                    } else {
                        non_matching_symbols.push(symbol.clone());
                    }
                }
                /*Symbol::ModuleDefinition(m) => {
                    if let Ok(value) = m.call(&self.argument_list, context) {
                        return Ok(Some(Value::Node(value)));
                    } else {
                        non_matching_symbols.push(symbol.clone());
                    }
                }*/
                symbol => unimplemented!("Call::eval for {symbol:?}"),
            }
        }

        if non_matching_symbols.is_empty() {
            println!("No matching symbol found for `{}`", self.name);
            return Ok(None);
        } else {
            println!("No matching symbol found for `{}`. Candidates:", self.name);
            for symbol in non_matching_symbols {
                println!("\t{} => {:#?}", symbol.name(), symbol);
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
