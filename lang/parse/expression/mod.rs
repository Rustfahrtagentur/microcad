// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD parser entities related to expressions

mod expression_list;
mod list_expression;
mod nested;
mod nested_item;
mod tuple_expression;

pub use expression_list::*;
pub use list_expression::*;
pub use nested::*;
pub use nested_item::*;
pub use tuple_expression::*;

use crate::{eval::*, parse::*, parser::*, src_ref::*};

lazy_static::lazy_static! {
    /// Expression parser
    static ref PRATT_PARSER: pest::pratt_parser::PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op,PrattParser};
        use Assoc::*;
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(r#union, Left) | Op::infix(intersection, Left))
            .op(Op::infix(power_xor, Left))
            .op(Op::infix(greater_than, Left) | Op::infix(less_than, Left))
            .op(Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left))
            .op(Op::infix(near, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(method_call))
            .op(Op::postfix(list_element_access))
            .op(Op::postfix(tuple_element_access))
    };
}

/// Expressions
#[derive(Clone, Debug)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    Invalid,
    /// An integer, float, color or bool literal: 1, 1.0, #00FF00, false
    Literal(Literal),
    /// A string that contains format expressions: "value = {a}"
    FormatString(FormatString),
    /// A list: [a, b, c]
    ListExpression(ListExpression),
    /// A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A list whitespace separated of nested items: `translate() rotate()`, `b c`, `a b() {}`
    Nested(Nested),
    /// A binary operation: a + b
    BinaryOp {
        /// Left-hand side
        lhs: Box<Expression>,
        /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
        op: String,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// A unary operation: !a
    UnaryOp {
        /// Operator ('+', '-', '!')
        op: String,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// Access an element of a list (`a[0]`) or a tuple (`a.0` or `a.b`)
    ListElementAccess(Box<Expression>, Box<Expression>, SrcRef),
    /// Access an element of a named tuple: `a.b`
    NamedTupleElementAccess(Box<Expression>, Identifier, SrcRef),
    /// Access an element of an unnamed tuple: `a.0`
    UnnamedTupleElementAccess(Box<Expression>, u32, SrcRef),
    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, MethodCall, SrcRef),
}

impl SrcReferrer for Expression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Invalid => SrcRef(None),
            Self::Literal(l) => l.src_ref(),
            Self::FormatString(fs) => fs.src_ref(),
            Self::ListExpression(le) => le.src_ref(),
            Self::TupleExpression(te) => te.src_ref(),
            Self::Nested(n) => n.src_ref().clone(),
            Self::BinaryOp {
                lhs: _,
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::UnaryOp {
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::ListElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::NamedTupleElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::UnnamedTupleElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::MethodCall(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::FormatString(format_string) => write!(f, "{format_string}"),
            Self::ListExpression(list_expression) => write!(f, "{list_expression}"),
            Self::TupleExpression(tuple_expression) => write!(f, "{tuple_expression}"),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({lhs} {op} {rhs})"),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({op}{rhs})"),
            Self::ListElementAccess(lhs, rhs, _) => write!(f, "{lhs}[{rhs}]"),
            Self::NamedTupleElementAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::UnnamedTupleElementAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs}.{method_call}"),
            Self::Nested(nested) => write!(f, "{nested}"),
            _ => unimplemented!(),
        }
    }
}

impl Expression {
    /// Generate literal from string
    pub fn literal_from_str(s: &str) -> ParseResult<Self> {
        use std::str::FromStr;
        if s.len() > 1 && s.starts_with('"') && s.ends_with('"') {
            Ok(Self::FormatString(FormatString::from_str(s)?))
        } else {
            Ok(Self::Literal(Literal::from_str(s)?))
        }
    }

    /// If the expression consists of a single identifier, e.g. `a`
    pub fn single_identifier(&self) -> Option<Identifier> {
        match &self {
            Self::Nested(nested) => nested.single_identifier(),
            _ => None,
        }
    }
}

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        use crate::diag::PushDiag;

        match self {
            Self::Literal(literal) => Literal::eval(literal, context),
            Self::FormatString(format_string) => FormatString::eval(format_string, context),
            Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            Self::TupleExpression(tuple_expression) => {
                TupleExpression::eval(tuple_expression, context)
            }
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;
                if lhs.is_invalid() || rhs.is_invalid() {
                    return Ok(Value::Invalid);
                }

                match op.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    "^" => unimplemented!(), // lhs.pow(&rhs),
                    "&" => lhs & rhs,
                    "|" => lhs | rhs,
                    ">" => Ok(Value::Bool(Refer::new(lhs > rhs, SrcRef::merge(lhs, rhs)))),
                    "<" => Ok(Value::Bool(Refer::new(lhs < rhs, SrcRef::merge(lhs, rhs)))),
                    "≤" => Ok(Value::Bool(Refer::new(lhs <= rhs, SrcRef::merge(lhs, rhs)))),
                    "≥" => Ok(Value::Bool(Refer::new(lhs >= rhs, SrcRef::merge(lhs, rhs)))),
                    "~" => todo!("implement near ~="),
                    "=" => Ok(Value::Bool(Refer::new(lhs == rhs, SrcRef::merge(lhs, rhs)))),
                    "!=" => Ok(Value::Bool(Refer::new(lhs != rhs, SrcRef::merge(lhs, rhs)))),
                    _ => unimplemented!("{op:?}"),
                }
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                let rhs = rhs.eval(context)?;
                match op.as_str() {
                    "-" => -rhs.clone(),
                    _ => unimplemented!(),
                }
            }
            Self::ListElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index.value as usize;
                        if index < list.len() {
                            Ok(list.get(index).unwrap().clone())
                        } else {
                            context.error(
                                self,
                                Box::new(EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                }),
                            )?;
                            Ok(Value::Invalid)
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::NamedTupleElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                match lhs {
                    Value::NamedTuple(tuple) => {
                        let value = tuple.get(rhs).unwrap();
                        Ok(value.clone())
                    }
                    Value::Node(node) => match node.fetch(&rhs.to_string().into()) {
                        Some(symbol) => match symbol.as_ref() {
                            Symbol::Value(_, value) => Ok(value.clone()),
                            _ => unimplemented!(),
                        },
                        None => {
                            context.error(self, Box::new(EvalError::UnknownField(rhs.clone())))?;
                            Ok(Value::Invalid)
                        }
                    },
                    _ => unimplemented!(),
                }
            }
            Self::MethodCall(lhs, method_call, _) => method_call.eval(context, lhs),
            Self::Nested(nested) => match nested.eval(context)? {
                Some(value) => Ok(value),
                None => Ok(Value::Invalid),
            },
            _ => unimplemented!(),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        if pair.as_rule() == Rule::expression_no_semicolon {
            return Ok(Self::Nested(Nested::parse(pair)?));
        }

        let mut error: Option<ParseError> = None;
        let result = PRATT_PARSER
            .map_primary(|primary| {
                match (
                    Pair::new(primary.clone(), pair.source_hash()),
                    primary.as_rule(),
                ) {
                    (primary, Rule::literal) => match Literal::parse(primary) {
                        Ok(literal) => Self::Literal(literal),
                        Err(e) => {
                            error = Some(e);
                            Self::Invalid
                        }
                    },
                    (primary, Rule::expression) => Self::parse(primary).unwrap(),
                    (primary, Rule::list_expression) => {
                        Self::ListExpression(ListExpression::parse(primary).unwrap())
                    }
                    (primary, Rule::tuple_expression) => {
                        Self::TupleExpression(TupleExpression::parse(primary).unwrap())
                    }
                    (primary, Rule::format_string) => {
                        Self::FormatString(FormatString::parse(primary).unwrap())
                    }
                    (primary, Rule::nested) => Self::Nested(Nested::parse(primary).unwrap()),
                    rule => unreachable!(
                        "Expression::parse expected atom, found {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    ),
                }
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => "+",
                    Rule::subtract => "-",
                    Rule::multiply => "*",
                    Rule::divide => "/",
                    Rule::r#union => "|",
                    Rule::intersection => "&",
                    Rule::power_xor => "^",
                    Rule::greater_than => ">",
                    Rule::less_than => "<",
                    Rule::less_equal => "≤",
                    Rule::greater_equal => "≥",
                    Rule::equal => "=",
                    Rule::near => "~",
                    Rule::not_equal => "!=",
                    Rule::and => "&",

                    rule => unreachable!(
                        "Expression::parse expected infix operation, found {:?}",
                        rule
                    ),
                };
                Self::BinaryOp {
                    lhs: Box::new(lhs),
                    op: op.into(),
                    rhs: Box::new(rhs),
                    src_ref: pair.clone().into(),
                }
            })
            .map_prefix(|op, rhs| {
                let op = match op.as_rule() {
                    Rule::unary_minus => '-',
                    Rule::unary_plus => '+',
                    Rule::unary_not => '!',
                    _ => unreachable!(),
                };

                Self::UnaryOp {
                    op: op.into(),
                    rhs: Box::new(rhs),
                    src_ref: pair.clone().into(),
                }
            })
            .map_postfix(|lhs, op| {
                match (Pair::new(op.clone(), pair.source_hash()), op.as_rule()) {
                    (op, Rule::list_element_access) => Self::ListElementAccess(
                        Box::new(lhs),
                        Box::new(Self::parse(op).unwrap()),
                        pair.clone().into(),
                    ),
                    (op, Rule::tuple_element_access) => {
                        let op = op.inner().next().unwrap();
                        match op.as_rule() {
                            Rule::identifier => Self::NamedTupleElementAccess(
                                Box::new(lhs),
                                Identifier::parse(op).unwrap(),
                                pair.clone().into(),
                            ),
                            Rule::int => Self::UnnamedTupleElementAccess(
                                Box::new(lhs),
                                op.as_str().parse().unwrap(),
                                pair.clone().into(),
                            ),
                            rule => unreachable!("Expected identifier or int, found {:?}", rule),
                        }
                    }
                    (op, Rule::method_call) => Self::MethodCall(
                        Box::new(lhs),
                        MethodCall::parse(op).unwrap(),
                        pair.clone().into(),
                    ),
                    rule => {
                        unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                    }
                }
            })
            .parse(pair.pest_pair().clone().into_inner());

        match error {
            Some(e) => Err(e),
            None => Ok(result),
        }
    }
}

#[test]
fn list_expression() {
    use crate::eval::*;

    let mut context = Context::default();

    // Simple list expression with 3 elements
    run_expression_test("[1,2,3]", &mut context, |e| {
        if let Ok(Value::List(list)) = e {
            assert_eq!(list.len(), 3);
        } else {
            panic!("Expected list value: {:?}", e);
        }
    });

    // Accessing the third element of a list
    run_expression_test("[1.0,2.0,3.0][2]", &mut context, |e| {
        if let Ok(Value::Scalar(n)) = e {
            assert_eq!(*n, 3.0);
        } else {
            panic!("Expected scalar value: {:?}", e);
        }
    });

    // Test out of bounds access
    run_expression_test("[1.0,2.0,3.0][3]", &mut context, |e| {
        if let Err(EvalError::ListIndexOutOfBounds { index, len }) = e {
            assert_eq!(index, 3);
            assert_eq!(len, 3);
        }
    });

    // Return the length of a list
    run_expression_test("[1.0,2.0,3.0].len()", &mut context, |e| {
        if let Ok(Value::Integer(n)) = e {
            assert_eq!(*n, 3);
        }
    });
}

#[cfg(test)]
fn run_expression_test(
    expr: &str,
    context: &mut crate::eval::Context,
    evaluator: impl FnOnce(EvalResult<crate::eval::Value>),
) {
    use crate::parser::{Parser, Rule};
    use pest::Parser as _;

    let pair = Pair::new(
        Parser::parse(Rule::expression, expr)
            .unwrap()
            .next()
            .unwrap(),
        0,
    );

    let expr = Expression::parse(pair).unwrap();
    let new_expr = expr.eval(context);

    evaluator(new_expr);
}

#[test]
fn operators() {
    let mut context = Context::default();
    run_expression_test("4", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(*num, 4.0);
        }
    });
    run_expression_test("4 * 4", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(*num, 16.0);
        }
    });
    run_expression_test("4 * (4 + 4)", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(*num, 32.0);
        }
    });
    run_expression_test("10.0 / 2.5 + 6", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(*num, 10.0);
        }
    });
}

#[test]
fn conditions() {
    let mut context = Context::default();

    run_expression_test("4 < 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(*b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 > 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(!*b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 == 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(!*b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 != 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(*b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
}
/*
#[test]
fn basic_context() {
    let mut context = Context::default();
    context.insert("a", Value::Scalar(4.0));
    context.insert("b", Value::Scalar(5.0));

    run_expression_test("a + b", Some(&context), |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 9.0);
        }
    });
    run_expression_test("a < b", Some(&context), |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(b);
        } else {
            panic!("Expected boolean value");
        }
    });
    run_expression_test("a + b + c", Some(&context), |e| {
        if let Err(eval::Error::UnknownQualifiedName(qualified_name)) = e {
            assert_eq!(qualified_name, "c".into());
        }
    });
}

#[test]
fn nested_context() {
    let mut context = Context::default();
    context.insert("a", Value::Scalar(4.0));S
    context.insert("b", Value::Scalar(5.0));

    // Enter a new scope
    context.push();
    context.insert("a", Value::String("Hello".into()));
    context.insert("b", Value::String("World".into()));

    run_expression_test("a + b", Some(&context), |e| {
        if let Ok(Value::String(s)) = e {
            assert_eq!(&s, "HelloWorld");
        }
    });

    context.pop();

    run_expression_test("a + b", Some(&context), |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 9.0);
        }
    });
}*/
