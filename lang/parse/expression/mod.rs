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

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

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
    // A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A list whitespace separated of nested items: `translate() rotate()`, `b c`, `a b() {}`
    Nested(Nested),
    /// A binary operation: a + b
    BinaryOp {
        lhs: Box<Expression>,
        /// '+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|'
        op: char,
        rhs: Box<Expression>,
        src_ref: SrcRef,
    },
    /// A unary operation: !a
    UnaryOp {
        /// '+', '-', '!'
        op: char,
        rhs: Box<Expression>,
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
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::FormatString(format_string) => write!(f, "{}", format_string),
            Self::ListExpression(list_expression) => write!(f, "{}", list_expression),
            Self::TupleExpression(tuple_expression) => write!(f, "{}", tuple_expression),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({} {} {})", lhs, op, rhs),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({}{})", op, rhs),
            Self::ListElementAccess(lhs, rhs, _) => write!(f, "{}[{}]", lhs, rhs),
            Self::NamedTupleElementAccess(lhs, rhs, _) => write!(f, "{}.{}", lhs, rhs),
            Self::UnnamedTupleElementAccess(lhs, rhs, _) => write!(f, "{}.{}", lhs, rhs),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{}.{}", lhs, method_call),
            Self::Nested(nested) => write!(f, "{nested}"),
            _ => unimplemented!(),
        }
    }
}

impl Expression {
    pub fn literal(literal: Literal) -> Self {
        Self::Literal(literal)
    }

    pub fn literal_from_str(s: &str) -> std::result::Result<Self, anyhow::Error> {
        use std::str::FromStr;
        if s.starts_with('"') && s.ends_with('"') {
            return Ok(Self::FormatString(FormatString::from_str(s)?));
        }
        Ok(Self::Literal(Literal::from_str(s)?))
    }
}

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
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

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '^' => unimplemented!(), // lhs.pow(&rhs),
                    '>' => lhs.greater_than(&rhs).map(Value::Bool),
                    '<' => lhs.less_than(&rhs).map(Value::Bool),
                    '≤' => lhs.less_than_or_equal(&rhs).map(Value::Bool),
                    '≥' => lhs.greater_than_or_equal(&rhs).map(Value::Bool),
                    '=' => Ok(Value::Bool(lhs.eq(&rhs))),
                    '≠' => Ok(Value::Bool(!lhs.eq(&rhs))),
                    _ => unimplemented!(),
                }
                .map_err(EvalError::ValueError)
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                let rhs = rhs.eval(context)?;

                match op {
                    '-' => rhs.neg(),
                    _ => unimplemented!(),
                }
                .map_err(EvalError::ValueError)
            }
            Self::ListElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index as usize;
                        if index < list.len() {
                            Ok(list.get(index).unwrap().clone())
                        } else {
                            Err(EvalError::ListIndexOutOfBounds {
                                index,
                                len: list.len(),
                            })
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::MethodCall(lhs, method_call, _) => {
                let name: &str = &method_call.name.to_string();

                match lhs.eval(context)? {
                    Value::List(list) => match name {
                        "len" => Ok(Value::Integer(list.len() as i64)),
                        _ => Err(EvalError::UnknownMethod(name.into())),
                    },
                    _ => Err(EvalError::UnknownMethod(name.into())),
                }
            }
            Self::Nested(nested) => nested.eval(context),
            _ => unimplemented!(),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut error: Option<ParseError> = None;
        let result = PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::literal => match Literal::parse(primary) {
                    Ok(literal) => Self::Literal(literal),
                    Err(e) => {
                        error = Some(e);
                        Self::Invalid
                    }
                },
                Rule::expression => Self::parse(primary).unwrap(),
                Rule::list_expression => {
                    Self::ListExpression(ListExpression::parse(primary).unwrap())
                }
                Rule::tuple_expression => {
                    Self::TupleExpression(TupleExpression::parse(primary).unwrap())
                }
                Rule::format_string => Self::FormatString(FormatString::parse(primary).unwrap()),
                Rule::nested => Self::Nested(Nested::parse(primary).unwrap()),
                rule => unreachable!(
                    "Expression::parse expected atom, found {:?} {:?}",
                    rule,
                    pair.as_span().as_str()
                ),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => '+',
                    Rule::subtract => '-',
                    Rule::multiply => '*',
                    Rule::divide => '/',
                    Rule::r#union => '|',
                    Rule::intersection => '&',
                    Rule::power_xor => '^',
                    Rule::greater_than => '>',
                    Rule::less_than => '<',
                    Rule::less_equal => '≤',
                    Rule::greater_equal => '≥',
                    Rule::equal => '=',
                    Rule::not_equal => '≠',
                    Rule::and => '&',

                    rule => unreachable!(
                        "Expression::parse expected infix operation, found {:?}",
                        rule
                    ),
                };
                Self::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
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
                    op,
                    rhs: Box::new(rhs),
                    src_ref: pair.clone().into(),
                }
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::list_element_access => Self::ListElementAccess(
                    Box::new(lhs),
                    Box::new(Self::parse(op).unwrap()),
                    pair.clone().into(),
                ),
                Rule::tuple_element_access => {
                    let op = op.into_inner().next().unwrap();
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
                Rule::method_call => Self::MethodCall(
                    Box::new(lhs),
                    MethodCall::parse(op).unwrap(),
                    pair.clone().into(),
                ),
                rule => {
                    unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                }
            })
            .parse(pair.clone().into_inner());

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
            assert_eq!(n, 3.0);
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
            assert_eq!(n, 3);
        }
    });
}

#[cfg(test)]
fn run_expression_test(
    expr: &str,
    context: &mut crate::eval::Context,
    evaluator: impl FnOnce(Result<crate::eval::Value>),
) {
    use crate::parser::{Parser, Rule};
    use pest::Parser as _;

    let pair = Parser::parse(Rule::expression, expr)
        .unwrap()
        .next()
        .unwrap();

    let expr = Expression::parse(pair).unwrap();
    let new_expr = expr.eval(context);

    evaluator(new_expr);
}

#[test]
fn operators() {
    let mut context = Context::default();
    run_expression_test("4", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 4.0);
        }
    });
    run_expression_test("4 * 4", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 16.0);
        }
    });
    run_expression_test("4 * (4 + 4)", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 32.0);
        }
    });
    run_expression_test("10.0 / 2.5 + 6", &mut context, |e| {
        if let Ok(Value::Scalar(num)) = e {
            assert_eq!(num, 10.0);
        }
    });
}

#[test]
fn conditions() {
    let mut context = Context::default();

    run_expression_test("4 < 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 > 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(!b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 == 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(!b);
        } else {
            panic!("Expected boolean value: {:?}", e);
        }
    });
    run_expression_test("4 != 5", &mut context, |e| {
        if let Ok(Value::Bool(b)) = e {
            assert!(b);
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
