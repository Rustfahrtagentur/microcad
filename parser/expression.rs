use crate::call::CallArgumentList;
use crate::eval::{self, Context, Eval};
use crate::format_string::FormatString;
use crate::langtype::Type;
use crate::list::ListExpression;
use crate::literal::NumberLiteral;
use crate::parser::*;
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
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(list_element_access))
            .op(Op::postfix(tuple_element_access))
            .op(Op::postfix(call_op))
    };
}

#[derive(Default, Clone)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,

    /// A string literal. The .0 is the content of the string, without the quotes
    StringLiteral(String),

    /// Number with an optional unit: 4.0mm, 3.0
    NumberLiteral(NumberLiteral),

    /// Boolean: true, false
    BoolLiteral(bool),

    /// A string that contains format expressions: "value = {a}"
    FormatString(FormatString),

    /// A list: [a, b, c]
    ListExpression(ListExpression),

    //    TupleExpression(TupleExpression),

    //    FunctionCall(FunctionCall),

    //    QualifiedName(QualifiedName)
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

    /// First expression must evaluate to `ModuleRef` or `FunctionRef`
    CallOp(Box<Expression>, crate::call::CallArgumentList),
}

impl Expression {}

/// Rules for operator +
impl std::ops::Add for Box<Expression> {
    type Output = Result<Box<Expression>, eval::Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.as_ref(), rhs.as_ref()) {
            (Expression::NumberLiteral(lhs), Expression::NumberLiteral(rhs)) => match lhs + rhs {
                Some(result) => Ok(Box::new(Expression::NumberLiteral(result))),
                None => Err(eval::Error::InvalidOperation),
            },
            _ => Err(eval::Error::InvalidOperation),
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for Box<Expression> {
    type Output = Result<Box<Expression>, eval::Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.as_ref(), rhs.as_ref()) {
            (Expression::NumberLiteral(lhs), Expression::NumberLiteral(rhs)) => match lhs - rhs {
                Some(result) => Ok(Box::new(Expression::NumberLiteral(result))),
                None => Err(eval::Error::InvalidOperation),
            },
            _ => Err(eval::Error::InvalidOperation),
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for Box<Expression> {
    type Output = Result<Box<Expression>, eval::Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.as_ref(), rhs.as_ref()) {
            (Expression::NumberLiteral(lhs), Expression::NumberLiteral(rhs)) => match lhs * rhs {
                Some(result) => Ok(Box::new(Expression::NumberLiteral(result))),
                None => Err(eval::Error::InvalidOperation),
            },
            _ => Err(eval::Error::InvalidOperation),
        }
    }
}

/// Rules for operator /
impl std::ops::Div for Box<Expression> {
    type Output = Result<Box<Expression>, eval::Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.as_ref(), rhs.as_ref()) {
            (Expression::NumberLiteral(lhs), Expression::NumberLiteral(rhs)) => match lhs / rhs {
                Some(result) => Ok(Box::new(Expression::NumberLiteral(result))),
                None => Err(eval::Error::InvalidOperation),
            },
            _ => Err(eval::Error::InvalidOperation),
        }
    }
}

impl Eval for Expression {
    fn eval(self, context: Option<&Context>) -> Result<Box<Self>, eval::Error> {
        match self {
            Self::NumberLiteral(_) | Self::StringLiteral(_) | Self::BoolLiteral(_) => {
                Ok(Box::new(self))
            }
            Self::FormatString(format_string) => FormatString::eval(format_string, context),
            Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            Self::BinaryOp { lhs, op, rhs } => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    _ => unimplemented!(),
                }
            }
            Self::ElementAccess(lhs, rhs) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expression::ListExpression(list), Expression::NumberLiteral(index)) => {
                        let index = index.value() as usize;
                        if index < list.len() {
                            Ok(Box::new(list.get(index).unwrap().clone()))
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
            _ => Err(eval::Error::InvalidOperation),
        }
    }

    /// The type this expression will evaluate to
    fn eval_type(&self, context: Option<&Context>) -> Result<Type, eval::Error> {
        match &self {
            Self::NumberLiteral(n) => n.eval_type(context),
            Self::StringLiteral(_) => Ok(Type::String),
            Self::BoolLiteral(_) => Ok(Type::Bool),
            Self::ListExpression(list) => list.eval_type(context),
            _ => Err(eval::Error::InvalidType),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::literal => {
                    let inner = primary.into_inner().next().unwrap();

                    match inner.as_rule() {
                        Rule::number_literal => {
                            let number_literal = NumberLiteral::parse(inner).unwrap();
                            Expression::NumberLiteral(number_literal)
                        }
                        rule => unreachable!("Expr::parse expected literal, found {:?}", rule),
                    }
                }
                Rule::number_literal => {
                    let number_literal = NumberLiteral::parse(primary).unwrap();
                    Expression::NumberLiteral(number_literal)
                }
                Rule::expression => Self::parse(primary).unwrap(),
                Rule::list_expression => {
                    println!("Parsing list expression: {}", primary.as_str());
                    Expression::ListExpression(ListExpression::parse(primary).unwrap())
                }
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => '+',
                    Rule::subtract => '-',
                    Rule::multiply => '*',
                    Rule::divide => '/',
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };
                Expression::BinaryOp {
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

                Expression::UnaryOp {
                    op,
                    rhs: Box::new(rhs),
                }
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::list_element_access | Rule::tuple_element_access => {
                    Expression::ElementAccess(Box::new(lhs), Box::new(Self::parse(op).unwrap()))
                }
                Rule::call_op => {
                    Expression::CallOp(Box::new(lhs), CallArgumentList::parse(op).unwrap())
                }
                rule => {
                    unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                }
            })
            .parse(pair.into_inner()))
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

    pub fn common_eval_type(&self, context: Option<&Context>) -> Result<Type, crate::eval::Error> {
        let types = self
            .0
            .iter()
            .map(|expr| expr.eval_type(context))
            .collect::<Result<Vec<Type>, crate::eval::Error>>()?;
        Ok(types.into_iter().fold(Type::Invalid, |acc, ty| {
            if acc == Type::Invalid {
                ty
            } else if acc == ty {
                acc
            } else {
                Type::Invalid
            }
        }))
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
        context: Option<&Context>,
        evaluator: impl FnOnce(&Expression),
    ) {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(Rule::expression, expr)
            .unwrap()
            .next()
            .unwrap();

        let expr = Expression::parse(pair).unwrap();
        let new_expr = expr.eval(context).unwrap();

        evaluator(new_expr.as_ref());
    }

    #[test]
    fn operators() {
        run_expression_test("4", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 4.0);
            }
        });
        run_expression_test("4 * 4", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 16.0);
            }
        });
        run_expression_test("4 * (4 + 4)", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 32.0);
            }
        });
        run_expression_test("10.0 / 2.5 + 6", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 10.0);
            }
        });
    }

    #[test]
    fn list_expression() {
        // Simple list expression with 3 elements
        run_expression_test("[1,2,3]", None, |e| {
            if let Expression::ListExpression(list) = e {
                assert_eq!(list.len(), 3);
            }
        });

        // Accessing the third element of a list
        run_expression_test("[1.0,2.0,3.0][2]", None, |e| {
            if let Expression::NumberLiteral(n) = e {
                assert_eq!(n.value(), 3.0);
            }
        });
    }
}
