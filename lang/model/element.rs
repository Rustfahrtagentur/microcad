// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use crate::{builtin::*, model::*, syntax::*, value::*};
use strum::IntoStaticStr;

/// An element defines the entity of a [`Model`].
#[derive(Clone, IntoStaticStr, Debug, Default)]
pub enum Element {
    #[default]
    /// A group element is created by a body `{}`.
    Group,

    /// A workpiece that holds properties.
    ///
    /// A workpiece is created by workbenches.
    Workpiece(Workpiece),

    /// A built-in workpiece.
    ///
    /// A workpiece is created by workbenches.
    BuiltinWorkpiece(BuiltinWorkpiece),

    /// Multiplicity.
    Multiplicity,

    /// Created from `@input` marker. Will never be part of the final model.
    InputPlaceholder,
}

impl Element {
    /// Get output type of element.
    pub fn output_type(&self) -> OutputType {
        match self {
            Element::Workpiece(workpiece) => match workpiece.kind {
                WorkbenchKind::Sketch => OutputType::Geometry2D,
                WorkbenchKind::Part => OutputType::Geometry3D,
                WorkbenchKind::Operation => OutputType::NotDetermined,
            },
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Primitive2D => OutputType::Geometry2D,
                BuiltinWorkbenchKind::Primitive3D => OutputType::Geometry3D,
                BuiltinWorkbenchKind::Transform | BuiltinWorkbenchKind::Operation => {
                    builtin_workpiece.output_type
                }
            },
            Element::Group | Element::Multiplicity | Element::InputPlaceholder => {
                OutputType::NotDetermined
            }
        }
    }

    /// Check if an element is an operation.
    pub fn is_operation(&self) -> bool {
        match self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Primitive2D | BuiltinWorkbenchKind::Primitive3D => false,
                BuiltinWorkbenchKind::Operation | BuiltinWorkbenchKind::Transform => true,
            },
            Element::Multiplicity | Element::Group | Element::InputPlaceholder => false,
            Element::Workpiece(workpiece) => match workpiece.kind {
                WorkbenchKind::Part | WorkbenchKind::Sketch => false,
                WorkbenchKind::Operation => true,
            },
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        match &self {
            Element::Workpiece(workpiece) => write!(f, "{workpiece}"),
            Element::BuiltinWorkpiece(builtin_workpiece) => write!(f, "{builtin_workpiece}"),
            _ => write!(f, "{name}"),
        }
    }
}

impl std::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Element::Group => std::mem::discriminant(&Element::Group).hash(state),
            Element::Multiplicity => std::mem::discriminant(&Element::Multiplicity).hash(state),
            Element::InputPlaceholder => {
                std::mem::discriminant(&Element::InputPlaceholder).hash(state)
            }
            Element::Workpiece(workpiece) => workpiece.hash(state),
            Element::BuiltinWorkpiece(builtin_workpiece) => builtin_workpiece.hash(state),
        }
    }
}

impl PropertiesAccess for Element {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        match self {
            Self::Workpiece(workpiece) => workpiece.get_property(id),
            _ => unreachable!("not a workpiece element"),
        }
    }

    fn set_property(&mut self, id: Identifier, value: Value) -> Option<Value> {
        match self {
            Self::Workpiece(workpiece) => workpiece.set_property(id, value),
            _ => unreachable!("not a workpiece element"),
        }
    }

    fn get_properties(&self) -> Option<&Properties> {
        match self {
            Self::Workpiece(workpiece) => workpiece.get_properties(),
            _ => None,
        }
    }

    fn add_properties(&mut self, props: Properties) {
        match self {
            Self::Workpiece(workpiece) => {
                workpiece.add_properties(props);
            }
            _ => unreachable!("not a workpiece element"),
        }
    }
}
