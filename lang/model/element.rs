// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use crate::{
    eval::{BuiltinWorkbenchKind, BuiltinWorkpiece, BuiltinWorkpieceOutput},
    model::*,
    syntax::*,
    value::*,
};
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

    /// A special element after which children will be nested as siblings.
    ///
    /// This element is removed after the children have been inserted.
    ChildrenMarker,
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
                    OutputType::NotDetermined
                }
            },
            Element::Group | Element::ChildrenMarker => OutputType::NotDetermined,
        }
    }

    /// Fetch the local matrix
    pub fn get_affine_transform(&self) -> render::RenderResult<Option<AffineTransform>> {
        match &self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Transform => {
                    match (builtin_workpiece.f)(&builtin_workpiece.args)? {
                        BuiltinWorkpieceOutput::Transform(affine_transform) => {
                            Ok(Some(affine_transform))
                        }
                        _ => unreachable!(),
                    }
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    /// Check if an element is an operation.
    pub fn is_operation(&self) -> bool {
        match self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Primitive2D | BuiltinWorkbenchKind::Primitive3D => false,
                BuiltinWorkbenchKind::Operation | BuiltinWorkbenchKind::Transform => true,
            },
            Element::ChildrenMarker | Element::Group => false,
            Element::Workpiece(workpiece) => match workpiece.kind {
                WorkbenchKind::Part | WorkbenchKind::Sketch => false,
                WorkbenchKind::Operation => true,
            },
        }
    }

    /// Contains geometry.
    pub fn contains_geometry(&self) -> bool {
        match self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Primitive2D | BuiltinWorkbenchKind::Primitive3D => true,
                BuiltinWorkbenchKind::Operation | BuiltinWorkbenchKind::Transform => false,
            },
            Element::ChildrenMarker => true,
            Element::Workpiece(workpiece) => match workpiece.kind {
                WorkbenchKind::Part | WorkbenchKind::Sketch => false,
                WorkbenchKind::Operation => false,
            },
            Element::Group => false,
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

impl PropertiesAccess for Element {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        match self {
            Self::Workpiece(workpiece) => workpiece.get_property(id),
            _ => unreachable!("not a workpiece element"),
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
