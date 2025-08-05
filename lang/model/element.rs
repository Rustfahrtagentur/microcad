// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use crate::{model::*, syntax::*, value::*};
use microcad_core::*;
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

    /// A special element after which children will be nested as siblings.
    ///
    /// This element is removed after the children have been inserted.
    ChildrenMarker,

    /// An affine transform.
    Transform(AffineTransform),

    /// A 2D geometry.
    Primitive2D(std::rc::Rc<Geometry2D>),

    /// A 3D geometry.
    Primitive3D(std::rc::Rc<Geometry3D>),

    /// An operation that generates geometries from its children.
    Operation(std::rc::Rc<dyn Operation>),
}

impl Element {
    /// Check if this element can nest other elements.
    pub fn can_nest(&self) -> bool {
        match &self {
            Element::Workpiece(workpiece) => match workpiece.kind {
                WorkbenchKind::Part => false,
                WorkbenchKind::Sketch => false,
                WorkbenchKind::Operation => true,
            },
            Element::Group
            | Element::ChildrenMarker
            | Element::Primitive2D(_)
            | Element::Primitive3D(_) => false,
            Element::Transform(_) | Element::Operation(_) => true,
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        match &self {
            Element::Workpiece(workpiece) => write!(f, "{workpiece}"),
            Element::Primitive2D(primitive) => {
                write!(f, "{name}({primitive:?})")
            }
            Element::Primitive3D(primitive) => {
                write!(f, "{name}({primitive:?})")
            }
            Element::Operation(transformation) => {
                write!(f, "{name}({transformation:?})")
            }
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
