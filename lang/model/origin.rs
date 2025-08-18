// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model origin. Original source code information about a model.

use microcad_core::Mat4;

use crate::{model::OutputType, syntax::*, value::*};

/// The origin is the [`Symbol`] and [`Tuple`] from which the model has been created.
///
/// Most likely, this will eventually replace the [`model::Element`] enum.
#[derive(Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum Origin {
    #[default]
    /// A group: `{ .. }`.
    Group,
    /// A workbench from a definition: `sketch Circle(r: Length) { ... }`.
    Workbench {
        /// Fully qualified name. (Note: to be replaced by hash?)
        symbol: QualifiedName,
        /// Workbench kind.
        kind: WorkbenchKind,
        /// Arguments.
        arguments: Tuple,
    },
    /// A builtin workbench from a Rust struct.
    BuiltinWorkbench {
        /// Fully qualified name. (Note: to be replaced by hash?)
        symbol: QualifiedName,
        /// Workbench kind.
        kind: WorkbenchKind,
        /// Arguments.
        arguments: Tuple,
    },
    /// Source file origin.
    SourceFile,
}

impl Origin {
    /// Get the qualified name of the creator symbol.
    pub fn get_qualified_name(&self) -> Option<QualifiedName> {
        match &self {
            Origin::Workbench { symbol, .. } | Origin::BuiltinWorkbench { symbol, .. } => {
                Some(symbol.clone())
            }
            Origin::SourceFile | Origin::Group => None,
        }
    }

    /// Return the expected output type of the origin.
    pub fn expected_output_type(&self) -> OutputType {
        match self {
            Origin::Workbench { kind, .. } | Origin::BuiltinWorkbench { kind, .. } => {
                (*kind).into()
            }
            _ => OutputType::NotDetermined,
        }
    }
}
