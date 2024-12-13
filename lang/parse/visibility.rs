// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Visibility of an entity

use crate::{parse::*, parser::*};

/// Visibility of an entity
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Clone, Debug, Default)]
pub enum Visibility {
    /// Private visibility
    #[default]
    Private,
    /// Public visibility
    Public,
}

impl Parse for Visibility {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::visibility);

        let s = pair.as_str();
        match s {
            "pub" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => unreachable!("Invalid visibility"),
        }
    }
}
