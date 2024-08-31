mod call_argument;
mod call_argument_list;
mod method_call;

pub use call_argument::*;
pub use call_argument_list::*;
pub use method_call::*;

use crate::{eval::*, parse::*, parser::*};

#[derive(Clone, Debug, Default)]
pub struct Call {
    #[allow(dead_code)]
    pub name: QualifiedName,
    #[allow(dead_code)]
    pub argument_list: CallArgumentList,
}

impl Sym for Call {
    fn id(&self) -> Option<microcad_core::Id> {
        self.name.id()
    }
}

impl Parse for Call {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();

        Ok(Call {
            name: QualifiedName::parse(first)?,
            argument_list: match inner.next() {
                Some(pair) => CallArgumentList::parse(pair)?,
                None => CallArgumentList::default(),
            },
        })
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}

impl Eval for Call {
    type Output = Option<Value>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
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
                symbol => {
                    let s: &'static str = symbol.into();
                    unimplemented!("Symbol::{s}")
                }
            }
        }

        if non_matching_symbols.is_empty() {
            println!("No matching symbol found for `{}`", self.name);
            return Ok(None);
        } else {
            println!("No matching symbol found for `{}`. Candidates:", self.name);
            for symbol in &non_matching_symbols {
                let s: &'static str = symbol.into();
                println!(
                    "\t{} => {}",
                    if let Some(id) = &symbol.id() {
                        id.as_str()
                    } else {
                        "<unnamed>"
                    },
                    s
                );
            }
        }

        Err(EvalError::SymbolNotFound(
            self.id().expect("unnamed symbol not found)"),
        ))
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
