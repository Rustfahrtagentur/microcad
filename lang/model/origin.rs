// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model origin. Original source code information about a model.

use crate::{eval::*, resolve::*, src_ref::*, syntax::*, value::*};

/// The origin is the [`Symbol`] and [`Tuple`] from which the model has been created.
#[derive(
    Clone, Default, Debug, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
pub struct Origin {
    /// The original symbol that has been called.
    creator: Option<Link<Symbol, QualifiedName>>,

    /// The original arguments.
    pub arguments: Tuple,

    /// The original source file.
    pub source_file: Option<std::rc::Rc<SourceFile>>,

    /// Source code reference of the call.
    pub call_src_ref: SrcRef,
}

impl Origin {
    /// Create a default origin from arguments
    pub fn new(arguments: Tuple) -> Self {
        Self {
            creator: None,
            arguments,
            source_file: None,
            call_src_ref: SrcRef(None),
        }
    }
    /// Get creator, if available.
    ///
    /// If a creator is available returns a `Link` to it which might need to be
    /// loaded before accessing it (see [`Self::load_creator()`]).
    pub fn get_creator(&self) -> &Option<Link<Symbol, QualifiedName>> {
        &self.creator
    }
    /// Set a new creator.
    pub fn set_creator(&mut self, creator: Symbol) {
        self.creator = Some(creator.into())
    }
    /// Get the original symbol which might be loaded at first access from given `symbol_table`.
    pub fn load_creator(&mut self, symbol_table: &mut impl Lookup) -> EvalResult<Option<&Symbol>> {
        if let Some(creator) = &mut self.creator {
            Ok(Some(creator.load(|name| symbol_table.lookup(name))?))
        } else {
            Ok(None)
        }
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.creator {
            Some(creator) => {
                write!(
                    f,
                    "{symbol}{arguments}",
                    symbol = creator.full_name(),
                    arguments = self.arguments,
                )
            }
            None => Ok(()),
        }
    }
}
