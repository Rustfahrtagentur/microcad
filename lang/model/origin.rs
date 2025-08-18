// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model origin. Original source code information about a model.

use microcad_core::Mat4;

use crate::{syntax::*, value::*};

/// The origin is the [`Symbol`] and [`Tuple`] from which the model has been created.
#[derive(Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum Origin {
    #[default]
    Group,
    Workbench {
        symbol: QualifiedName, // Fully qualified name
        kind: WorkbenchKind,
        arguments: Tuple,
    },
    BuiltinWorkbench {
        symbol: QualifiedName, // Fully qualified name
        kind: WorkbenchKind,
        arguments: Tuple,
    },
    BuiltinTransform {
        symbol: QualifiedName,
        arguments: Tuple,
        matrix: Mat4,
    },
    SourceFile,
}

impl Origin {
    pub fn get_creator(&self) -> Option<QualifiedName> {
        match &self {
            Origin::Workbench { symbol, .. }
            | Origin::BuiltinWorkbench { symbol, .. }
            | Origin::BuiltinTransform { symbol, .. } => Some(symbol.clone()),
            Origin::SourceFile | Origin::Group => None,
        }
    }
}
