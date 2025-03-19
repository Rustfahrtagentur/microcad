// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter parser entities

mod parameter_list;

use crate::{ord_map::*, parse::*, parser::*, src_ref::*, r#type::*};

pub use parameter_list::*;

/// A parameter for a function or module definition
#[derive(Clone, Debug, Default)]
pub struct Parameter {
    /// Name of the parameter
    pub name: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<Expression>,
    /// Source code reference
    src_ref: SrcRef,
}

impl Parameter {
    /// Create new parameter
    pub fn new(
        name: Identifier,
        specified_type: Option<TypeAnnotation>,
        default_value: Option<Expression>,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            name,
            specified_type,
            default_value,
            src_ref,
        }
    }
}

impl SrcReferrer for Parameter {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.name.clone())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.name),
            (Some(t), None) => write!(f, "{}: {t}", self.name),
            (None, Some(v)) => write!(f, "{} = {v}", self.name),
            _ => Ok(()),
        }
    }
}

impl Syntax for Parameter {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Parameter '{}'", "", self.name)
    }
}

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
            name,
            specified_type,
            default_value,
            src_ref: pair.into(),
        })
    }
}

/// Short cut to create a `Parameter` instance
#[macro_export]
macro_rules! parameter {
    ($name:ident) => {
        microcad_lang::parse::Parameter::new(stringify!($name).into(), None, None, SrcRef(None))
    };
    ($name:ident: $ty:ident) => {
        microcad_lang::parse::Parameter::new(
            stringify!($name).into(),
            Some(microcad_lang::r#type::Type::$ty.into()),
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
