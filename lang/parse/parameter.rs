use crate::{parse::*, parser::*, ty::*};

/// Short cut to create a `ParameterList` instance
impl Parse for Parameter {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut default_value = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!(
                        "Unexpected token in parameter: {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    );
                }
            }
        }

        if specified_type.is_none() && default_value.is_none() {
            return Err(ParseError::ParameterMissingTypeOrValue(name.clone()));
        }

        Ok(Self {
            id: name,
            specified_type,
            default_value,
            src_ref: pair.into(),
        })
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.inner() {
            parameters
                .push(Parameter::parse(pair)?)
                .map_err(ParseError::DuplicateIdentifier)?;
        }

        Ok(parameters)
    }
}

/// Short cut to create a `Parameter` instance
#[macro_export]
macro_rules! parameter {
    ($name:ident) => {
        microcad_lang::parse::Parameter::new(stringify!($name).into(), None, None, SrcRef(None))
    };
    ($name:ident: $ty:ident) => {
        microcad_lang::syntax::Parameter::new(
            Identifier(microcad_lang::src_ref::Refer::none(
                stringify!($name).into(),
            )),
            Some(microcad_lang::ty::Type::$ty.into()),
            None,
            microcad_lang::src_ref::SrcRef(None),
        )
    };
    ($name:ident: $ty:ident = $value:expr) => {
        microcad_lang::parse::Parameter::new(
            stringify!($name).into(),
            Some(microcad_lang::r#type::Type::$ty.into()),
            Some(Expression::literal_from_str(stringify!($value)).expect("Invalid literal")),
            microcad_lang::src_ref::SrcRef(None),
        )
    };
}
