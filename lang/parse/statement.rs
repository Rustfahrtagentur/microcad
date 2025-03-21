use super::*;
use crate::{parser::*, *};

impl Parse for Assignment {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        Ok(Self {
            name,
            specified_type,
            value: value.expect(INTERNAL_PARSE_ERROR),
            src_ref: pair.into(),
        })
    }
}

impl Parse for IfStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut cond = Default::default();
        let mut body = Body::default();
        let mut body_else = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => cond = Expression::parse(pair)?,
                Rule::body => body = Body::parse(pair)?,
                Rule::body_else => {
                    body_else = Some(Body::parse(pair.clone())?);
                }
                rule => unreachable!("Unexpected rule in if, got {:?}", rule),
            }
        }

        Ok(IfStatement {
            cond,
            body,
            body_else,
            src_ref: pair.into(),
        })
    }
}

impl Parse for Marker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::marker_statement);
        Ok(Self {
            name: Identifier::parse(pair.inner().next().expect(INTERNAL_PARSE_ERROR))?,
            src_ref: pair.src_ref(),
        })
    }
}

impl Parse for Statement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::statement);
        let first = pair.inner().next().expect(INTERNAL_PARSE_ERROR);
        Ok(match first.as_rule() {
            Rule::module_definition => Self::Module(Rc::<ModuleDefinition>::parse(first)?),
            Rule::namespace_definition => Self::Namespace(Rc::<NamespaceDefinition>::parse(first)?),
            Rule::function_definition => Self::Function(Rc::<FunctionDefinition>::parse(first)?),
            Rule::module_init_definition => {
                Self::ModuleInit(Rc::new(ModuleInitDefinition::parse(first)?))
            }

            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::return_statement => Self::Return(ReturnStatement::parse(first)?),
            Rule::if_statement => Self::If(IfStatement::parse(first)?),
            Rule::marker_statement => Self::Marker(Marker::parse(first)?),

            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            Rule::expression | Rule::expression_no_semicolon => {
                Self::Expression(Expression::parse(first)?)
            }
            rule => unreachable!(
                "Unexpected module statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl Parse for ReturnStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut result = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => result = Some(Expression::parse(pair)?),
                rule => unreachable!("Unexpected rule in return, got {:?}", rule),
            }
        }

        Ok(ReturnStatement {
            result,
            src_ref: pair.into(),
        })
    }
}
