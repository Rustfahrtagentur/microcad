use std::rc::Rc;

use crate::call::Call;
use crate::eval::{self, Context, Eval};
use crate::format_string::FormatString;
use crate::identifier::QualifiedName;
use crate::list::ListExpression;
use crate::literal::Literal;
use crate::parser::*;
use crate::tuple::TupleExpression;
use crate::value::Value;
use pest::pratt_parser::PrattParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(greater_than, Left) | Op::infix(less_than, Left))
            .op(Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(list_element_access))
            .op(Op::postfix(tuple_element_access))
            .op(Op::postfix(method_call))
    };
}

#[derive(Clone)]
pub enum NestedItem {
    Call(Call),
    QualifiedName(QualifiedName),
    // TODO: ModuleInner(ModuleInner),
}

impl Parse for NestedItem {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::call => Call::parse(pair).map(NestedItem::Call),
            Rule::qualified_name => QualifiedName::parse(pair).map(NestedItem::QualifiedName),
            rule => unreachable!(
                "NestedItem::parse expected call or qualified name, found {:?}",
                rule
            ),
        }
    }
}

#[derive(Clone)]
pub struct Nested(Vec<NestedItem>);

impl Parse for Nested {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(Nested(
            pair.into_inner()
                .map(NestedItem::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Eval for Nested {
    type Output = crate::value::Value;

    fn eval(&self, context: &mut Context) -> Result<Value, eval::Error> {
        for item in &self.0 {
            match item {
                NestedItem::Call(call) => {
                    // Ok(call.eval(context)?);
                } //NestedItem::QualifiedName(qualified_name) => match context {
                //    Ok(qualified_name.eval(context)?)
                //}
                _ => todo!(),
            }
        }
        Err(eval::Error::Unknown)
    }
}

#[derive(Default, Clone)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
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
    },

    /// A unary operation: !a
    UnaryOp {
        /// '+', '-', '!'
        op: char,
        rhs: Box<Expression>,
    },

    /// A reference to a module, syntax node must contain a Module declaration
    ModuleRef(crate::syntax_tree::SyntaxNode),

    /// A reference to a function, syntax node must contain a Function declaration
    FunctionRef(crate::syntax_tree::SyntaxNode),

    /// Access an element of a list (`a[0]`) or a tuple (`a.0` or `a.b`)
    ElementAccess(Box<Expression>, Box<Expression>),

    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to `ModuleRef` or `FunctionRef`
    MethodCall(Box<Expression>, crate::call::MethodCall),
}

impl Expression {}

impl Eval for Expression {
    type Output = crate::value::Value;

    fn eval(&self, context: &mut Context) -> Result<Value, eval::Error> {
        match self {
            Self::Literal(literal) => Literal::eval(literal, context),
            Self::FormatString(format_string) => FormatString::eval(format_string, context),
            Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            Self::TupleExpression(tuple_expression) => {
                TupleExpression::eval(tuple_expression, context)
            }
            Self::BinaryOp { lhs, op, rhs } => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    '>' => lhs.greater_than(&rhs).map(Value::Bool),
                    '<' => lhs.less_than(&rhs).map(Value::Bool),
                    '≤' => lhs.less_than_or_equal(&rhs).map(Value::Bool),
                    '≥' => lhs.greater_than_or_equal(&rhs).map(Value::Bool),
                    '=' => Ok(Value::Bool(lhs.eq(&rhs))),
                    '≠' => Ok(Value::Bool(!lhs.eq(&rhs))),
                    _ => unimplemented!(),
                }
                .map_err(eval::Error::ValueError)
            }
            Self::ElementAccess(lhs, rhs) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index as usize;
                        if index < list.len() {
                            Ok(list.get(index).unwrap().clone())
                        } else {
                            Err(eval::Error::ListIndexOutOfBounds {
                                index,
                                len: list.len(),
                            })
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::MethodCall(lhs, method_call) => {
                let name: &str = &method_call.name.to_string();

                match lhs.eval(context)? {
                    Value::List(list) => match name {
                        "len" => Ok(Value::Integer(list.len() as i64)),
                        _ => Err(eval::Error::UnknownMethod(name.into())),
                    },
                    _ => Err(eval::Error::UnknownMethod(name.into())),
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
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
                rule => unreachable!("Expression::parse expected atom, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => '+',
                    Rule::subtract => '-',
                    Rule::multiply => '*',
                    Rule::divide => '/',
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
                }
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::list_element_access | Rule::tuple_element_access => {
                    Self::ElementAccess(Box::new(lhs), Box::new(Self::parse(op).unwrap()))
                }
                Rule::method_call => {
                    Self::MethodCall(Box::new(lhs), crate::call::MethodCall::parse(op).unwrap())
                }
                rule => {
                    unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                }
            })
            .parse(pair.into_inner());

        match error {
            Some(e) => Err(e),
            None => Ok(result),
        }
    }
}

#[derive(Default, Clone)]
pub struct ExpressionList(Vec<Expression>);

impl ExpressionList {
    pub fn new(v: Vec<Expression>) -> Self {
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Expression> {
        self.0.get(index)
    }
}

impl IntoIterator for ExpressionList {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Parse for ExpressionList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(Self(
            pair.into_inner()
                .map(|pair| Expression::parse(pair))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_expression_test(
        expr: &str,
        context: &mut Context,
        evaluator: impl FnOnce(Result<Value, eval::Error>),
    ) {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(Rule::expression, expr)
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
    fn list_expression() {
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
            if let Err(eval::Error::ListIndexOutOfBounds { index, len }) = e {
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
}
