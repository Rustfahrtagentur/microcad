use super::{
    call::*, format_string::*, identifier::*, list::*, literal::*, module::*, tuple::*, value::*,
};
use crate::{eval::*, parser::*, with_pair_ok};
use pest::pratt_parser::*;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Assoc::*;
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(union, Left) | Op::infix(intersection, Left))
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

#[derive(Clone, Debug)]
pub enum NestedItem {
    Call(Call),
    QualifiedName(QualifiedName),
    ModuleBody(ModuleBody),
}

impl Parse for NestedItem {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        match pair.clone().as_rule() {
            Rule::call => with_pair_ok!(
                NestedItem::Call(Call::parse(pair.clone())?.value().clone()),
                pair
            ),
            Rule::qualified_name => {
                with_pair_ok!(
                    NestedItem::QualifiedName(QualifiedName::parse(pair.clone())?.value().clone()),
                    pair
                )
            }
            Rule::module_body => {
                with_pair_ok!(
                    NestedItem::ModuleBody(ModuleBody::parse(pair.clone())?.value().clone()),
                    pair
                )
            }
            rule => unreachable!(
                "NestedItem::parse expected call or qualified name, found {:?}",
                rule
            ),
        }
    }
}

impl std::fmt::Display for NestedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NestedItem::Call(call) => write!(f, "{}", call),
            NestedItem::QualifiedName(qualified_name) => write!(f, "{}", qualified_name),
            NestedItem::ModuleBody(body) => write!(f, "{}", body),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Nested(Vec<NestedItem>);

impl Parse for Nested {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner().filter(|pair| {
            [Rule::qualified_name, Rule::call, Rule::module_body].contains(&pair.as_rule())
        }) {
            vec.push(NestedItem::parse(pair)?.value().clone());
        }

        with_pair_ok!(Nested(vec), pair)
    }
}

impl Eval for Nested {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let root = context.current_node();

        let mut values = Vec::new();
        for (index, item) in self.0.iter().enumerate() {
            match item {
                NestedItem::Call(call) => match call.eval(context)? {
                    Some(value) => values.push(value),
                    None => {
                        if index != 0 {
                            return Err(Error::CannotNestFunctionCall);
                        } else {
                            return Ok(Value::Scalar(0.0)); // @todo This is a hack. Return a Option::None here
                        }
                    }
                },
                NestedItem::QualifiedName(qualified_name) => {
                    let symbols = qualified_name.eval(context)?;

                    for symbol in symbols {
                        if let Symbol::Value(_, v) = symbol {
                            values.push(v.clone()); // Find first value only. @todo Backpropagation of values
                            break;
                        }
                    }
                }
                NestedItem::ModuleBody(body) => {
                    let new_node = body.eval(context)?;
                    new_node.detach();
                    values.push(Value::Node(new_node));
                }
            }
        }

        assert!(!values.is_empty());

        if values.len() == 1 {
            return Ok(values[0].clone());
        }

        // Finally, nest all nodes
        for value in values {
            match value {
                Value::Node(node) => {
                    node.detach();
                    let nested = context.append_node(node);
                    context.set_current_node(nested);
                }
                _ => {
                    return Err(Error::CannotNestFunctionCall);
                }
            }
        }

        context.set_current_node(root.clone());

        Ok(Value::Node(root.clone()))
    }
}

#[derive(Default, Clone, Debug)]
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
    /// Access an element of a list (`a[0]`) or a tuple (`a.0` or `a.b`)
    ListElementAccess(Box<Expression>, Box<Expression>),

    /// Access an element of a named tuple: `a.b`
    NamedTupleElementAccess(Box<Expression>, Identifier),

    /// Access an element of an unnamed tuple: `a.0`
    UnnamedTupleElementAccess(Box<Expression>, u32),

    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, MethodCall),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::FormatString(format_string) => write!(f, "{}", format_string),
            Self::ListExpression(list_expression) => write!(f, "{}", list_expression),
            Self::TupleExpression(tuple_expression) => write!(f, "{}", tuple_expression),
            Self::BinaryOp { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Self::UnaryOp { op, rhs } => write!(f, "({}{})", op, rhs),
            Self::ListElementAccess(lhs, rhs) => write!(f, "{}[{}]", lhs, rhs),
            Self::NamedTupleElementAccess(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Self::UnnamedTupleElementAccess(lhs, rhs) => write!(f, "{}.{}", lhs, rhs),
            Self::MethodCall(lhs, method_call) => write!(f, "{}.{}", lhs, method_call),
            Self::Nested(nested) => {
                let mut iter = nested.0.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for item in iter {
                        write!(f, " {}", item)?;
                    }
                }
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

impl Expression {}

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value, Error> {
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
                    '^' => unimplemented!(), // lhs.pow(&rhs),
                    '>' => lhs.greater_than(&rhs).map(Value::Bool),
                    '<' => lhs.less_than(&rhs).map(Value::Bool),
                    '≤' => lhs.less_than_or_equal(&rhs).map(Value::Bool),
                    '≥' => lhs.greater_than_or_equal(&rhs).map(Value::Bool),
                    '=' => Ok(Value::Bool(lhs.eq(&rhs))),
                    '≠' => Ok(Value::Bool(!lhs.eq(&rhs))),
                    _ => unimplemented!(),
                }
                .map_err(Error::ValueError)
            }
            Self::UnaryOp { op, rhs } => {
                let rhs = rhs.eval(context)?;

                match op {
                    '-' => rhs.neg(),
                    _ => unimplemented!(),
                }
                .map_err(Error::ValueError)
            }
            Self::ListElementAccess(lhs, rhs) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index as usize;
                        if index < list.len() {
                            Ok(list.get(index).unwrap().clone())
                        } else {
                            Err(Error::ListIndexOutOfBounds {
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
                        _ => Err(Error::UnknownMethod(name.into())),
                    },
                    _ => Err(Error::UnknownMethod(name.into())),
                }
            }
            Self::Nested(nested) => nested.eval(context),
            _ => unimplemented!(),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut error: Option<ParseError> = None;
        let result = PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::literal => match Literal::parse(primary) {
                    Ok(literal) => Self::Literal(literal.value().clone()),
                    Err(e) => {
                        error = Some(e);
                        Self::Invalid
                    }
                },
                Rule::expression => Self::parse(primary).unwrap().value().clone(),
                Rule::list_expression => {
                    Self::ListExpression(ListExpression::parse(primary).unwrap().value().clone())
                }
                Rule::tuple_expression => {
                    Self::TupleExpression(TupleExpression::parse(primary).unwrap().value().clone())
                }
                Rule::format_string => {
                    Self::FormatString(FormatString::parse(primary).unwrap().value().clone())
                }
                Rule::nested => Self::Nested(Nested::parse(primary).unwrap().value().clone()),
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
                    Rule::union => '|',
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
                Rule::list_element_access => Self::ListElementAccess(
                    Box::new(lhs),
                    Box::new(Self::parse(op).unwrap().value().clone()),
                ),
                Rule::tuple_element_access => {
                    let op = op.into_inner().next().unwrap();
                    match op.as_rule() {
                        Rule::identifier => Self::NamedTupleElementAccess(
                            Box::new(lhs),
                            Identifier::parse(op).unwrap().value().clone(),
                        ),
                        Rule::int => Self::UnnamedTupleElementAccess(
                            Box::new(lhs),
                            op.as_str().parse().unwrap(),
                        ),
                        rule => unreachable!("Expected identifier or int, found {:?}", rule),
                    }
                }
                Rule::method_call => Self::MethodCall(
                    Box::new(lhs),
                    MethodCall::parse(op).unwrap().value().clone(),
                ),
                rule => {
                    unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                }
            })
            .parse(pair.clone().into_inner());

        match error {
            Some(e) => Err(e),
            None => with_pair_ok!(result, pair),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct ExpressionList(Vec<Expression>);

impl std::ops::Deref for ExpressionList {
    type Target = Vec<Expression>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExpressionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ExpressionList {
    pub fn new(v: Vec<Expression>) -> Self {
        Self(v)
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut vec = Vec::new();

        for pair in pair.clone().into_inner() {
            vec.push(Expression::parse(pair)?.value().clone());
        }

        with_pair_ok!(Self(vec), pair)
    }
}

impl std::fmt::Display for ExpressionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", expr)?;
        }
        Ok(())
    }
}

#[cfg(test)]
fn run_expression_test(
    expr: &str,
    context: &mut Context,
    evaluator: impl FnOnce(Result<Value, Error>),
) {
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
        if let Err(Error::ListIndexOutOfBounds { index, len }) = e {
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
