// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use crate::{model::*, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// An element defines the entity of a [`Model`].
#[derive(Clone, IntoStaticStr, Debug)]
pub enum Element {
    /// An workpiece that contains children and holds properties.
    ///
    /// Workpiece can be created by builtins, assignments, expressions and workbenches.
    Workpiece(Properties),

    /// A special element after which children will be nested as siblings.
    ///
    /// This element is removed after the children have been inserted.
    ChildrenPlaceholder,

    /// An affine transform.
    Transform(AffineTransform),

    /// A 2D geometry.
    Primitive2D(std::rc::Rc<Geometry2D>),

    /// A 3D geometry.
    Primitive3D(std::rc::Rc<Geometry3D>),

    /// An operation that generates geometries from its children.
    Operation(std::rc::Rc<dyn Operation>),
}

impl Default for Element {
    fn default() -> Self {
        Element::Workpiece(Default::default())
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")?;

        match &self {
            Element::Operation(transformation) => {
                write!(f, "({transformation:?})")
            }
            Element::Primitive2D(primitive) => {
                write!(f, "({primitive:?})")
            }
            _ => Ok(()),
        }
    }
}

impl PropertiesAccess for Element {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        match self {
            Self::Workpiece(object) => object.get(id),
            _ => unreachable!("not an object element"),
        }
    }

    fn set_properties(&mut self, props: Properties) {
        match self {
            Self::Workpiece(p) => {
                p.extend(props.iter().map(|(id, prop)| (id.clone(), prop.clone())));
            }
            _ => unreachable!("not an object element"),
        }
    }
}
