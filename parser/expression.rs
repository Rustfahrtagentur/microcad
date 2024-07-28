use crate::call::CallArgumentList;
use crate::eval::{self, Context, Eval};
use crate::format_string::FormatString;
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
            .op(Op::postfix(element_access))
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
            Self::NumberLiteral(_) | Self::StringLiteral(_) => Ok(Box::new(self)),
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
            _ => Err(eval::Error::InvalidOperation),
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
                            let number_literal = Parser::number_literal(inner).unwrap();
                            Expression::NumberLiteral(number_literal)
                        }
                        rule => unreachable!("Expr::parse expected literal, found {:?}", rule),
                    }
                }
                Rule::number_literal => {
                    let number_literal = Parser::number_literal(primary).unwrap();
                    Expression::NumberLiteral(number_literal)
                }
                Rule::expression => Self::parse(primary).unwrap(),
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
                Rule::element_access => {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn run_operator_test(
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
        run_operator_test("4 * 4", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 16.0);
            }
        });
        run_operator_test("4 * (4 + 4)", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 32.0);
            }
        });
        run_operator_test("10.0 / 2.5 + 6", None, |e| {
            if let Expression::NumberLiteral(num) = e {
                assert_eq!(num.value(), 10.0);
            }
        });
    }
}
