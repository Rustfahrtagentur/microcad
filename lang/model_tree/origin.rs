// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node origin. Original source code information about a model node.

use crate::{
    resolve::{FullyQualify, Symbol},
    src_ref::SrcRef,
    syntax::SourceFile,
    value::*,
};

/// The origin is the [`Symbol`] and [`Tuple`] from which the node has been created.
#[derive(Clone, Default, Debug)]
pub struct ModelNodeOrigin {
    /// The original symbol that has been called.
    pub creator: Option<Symbol>,

    /// The original arguments.
    pub arguments: Tuple,

    /// The original source file.
    pub source_file: Option<std::rc::Rc<SourceFile>>,

    /// Source code reference of the call.
    pub call_src_ref: SrcRef,
}

impl std::fmt::Display for ModelNodeOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.creator {
            Some(creator) => {
                write!(
                    f,
                    "{symbol}({arguments})",
                    symbol = creator.full_name(),
                    arguments = self.arguments,
                )
            }
            None => Ok(()),
        }
    }
}
