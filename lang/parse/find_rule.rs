// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::parse::*;

/// Find rule with proper error handling.
///
/// - `pair`: Pair to solve
/// - `rule`: Rule to parse.
/// - `def`: if given returns default if not found
pub fn find_rule<T: crate::parser::Parse>(
    pair: &crate::parser::Pair<'_>,
    rule: crate::parser::Rule,
    def: Option<T>,
) -> ParseResult<T> {
    match pair
        .inner()
        .find(|pair| pair.as_rule() == rule)
        .map(T::parse)
    {
        Some(Ok(stmts)) => Ok(stmts),
        Some(Err(err)) => Err(err),
        None => {
            if let Some(def) = def {
                Ok(def)
            } else {
                Err(ParseError::NotAvailable)
            }
        }
    }
}

/// Find rule  or use default with proper error handling.
///
/// e.g.:
///
/// ```
/// let s : StatementList = crate::find_rule!(pair, StatementList, statement_list)
/// ```
#[macro_export]
macro_rules! find_rule {
    ($pair:ident, $rule:ident) => {
        find_rule(&$pair, Rule::$rule, Some(Default::default()))
    };
}

/// Find rule and return optional with proper error handling.
///
/// e.g.:
///
/// ```
/// let s : StatementList = crate::find_rule_opt!(pair, StatementList, statement_list)
/// ```
#[macro_export]
macro_rules! find_rule_opt {
    ($pair:ident, $rule:ident) => {
        match find_rule(&$pair, Rule::$rule, None) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    };
}

/// Find rule and return definition or [`ParseError::NotAvailable`] with proper error handling.
///
/// e.g.:
///
/// ```
/// let s : StatementList = crate::find_rule_opt!(pair, StatementList, statement_list)
/// ```
#[macro_export]
macro_rules! find_rule_exact {
    ($pair:ident, $rule:ident) => {
        find_rule(&$pair, Rule::$rule, None)
    };
}
