// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function signature parser entity

use crate::{errors::*, parse::*, parser::*, r#type::*, src_ref::*};

/// Parameters and return type of a function
#[derive(Clone, Debug)]
pub struct FunctionSignature {
    /// Function's parameters
    pub parameters: ParameterList,
    /// Function's return type
    pub return_type: Option<TypeAnnotation>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for FunctionSignature {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionSignature {
    /// Create new function signature
    pub fn new(parameters: ParameterList, return_type: Option<Type>, src_ref: SrcRef) -> Self {
        Self {
            parameters,
            return_type: return_type.map(|r| TypeAnnotation(Refer::none(r))),
            src_ref,
        }
    }

    /// Get parameter by name
    pub fn parameter_by_name(&self, name: &Identifier) -> Option<&Parameter> {
        self.parameters.iter().find(|arg| arg.name == *name)
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut parameters = ParameterList::default();
        let mut return_type = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::r#type => return_type = Some(TypeAnnotation::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
            return_type,
            src_ref: pair.into(),
        })
    }
}

/// Short cut to create a builtin function signature
#[macro_export]
macro_rules! function_signature {
    ($parameters:expr) => {
        microcad_lang::parse::function::FunctionSignature::new(
            $parameters,
            None,
            microcad_lang::src_ref::SrcRef(None),
        )
    };
    (($parameters:expr) -> $return_type:ident) => {
        microcad_lang::parse::function::FunctionSignature::new(
            $parameters,
            Some(Type::$return_type),
        )
    };
    () => {};
}
