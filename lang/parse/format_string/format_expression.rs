//! Format expression parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: FormatSpec,
    /// Expression to format
    pub expression: Box<Expression>,
    /// Source code reference
    src_ref: SrcRef,
}

impl Parse for FormatExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut spec = FormatSpec::default();
        let mut expression = None;
        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::format_spec => spec = FormatSpec::parse(pair)?,
                Rule::expression => expression = Some(Expression::parse(pair)?),
                _ => unreachable!(),
            }
        }
        if let Some(expression) = expression {
            Ok(Self {
                src_ref: SrcRef::merge(spec.src_ref(), expression.src_ref()),
                spec,
                expression: Box::new(expression),
            })
        } else {
            Err(ParseError::MissingFormatExpression)
        }
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        SrcRef::merge(&self.spec, self.expression.as_ref())
    }
}

impl Eval for FormatExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
        Ok(Value::String(Refer::new(
            format!("{}", self.expression.eval(context)?),
            SrcRef(None),
        )))
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", self.expression)
    }
}
