// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

#[macro_use]
mod call_argument_value;
mod call_argument;
mod call_argument_list;
mod call_argument_value_list;
mod call_trait;

pub use call_argument_value::*;
pub use call_argument_value_list::*;
pub use call_trait::*;

use crate::eval::*;
use crate::syntax::CallArgumentList;

use thiserror::Error;

impl CallArgumentList {
    /// Get matching arguments from parameter list
    pub fn get_matching_arguments(
        &self,
        _context: &mut EvalContext,
        _parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        todo!()
    }
}

/// An error that occurred when looking for matching arguments between a call and a parameter definition
#[derive(Error, Debug)]
pub enum MatchError {
    /// Duplicated argument
    #[error("Duplicated argument: {0}")]
    DuplicatedArgument(Id),
    /// Occurs when a parameter was given in a call but not in the definition
    #[error("Parameter `{0}` is not defined.")]
    ParameterNotDefined(Id),
    /// Mismatching type
    #[error("Type mismatch for parameter `{0}`: expected `{1}`, got {2}")]
    PositionalArgumentTypeMismatch(Id, Type, Type),
    /// Parameter required by definition but given in the call
    #[error("Missing parameter: {0}")]
    MissingParameter(Id),
}
