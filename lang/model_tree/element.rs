// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`ModelNode`]

use crate::{model_tree::*, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// An element defines the entity of a [`ModelNode`].
#[derive(Clone, IntoStaticStr, Debug)]
pub enum Element {
    /// An object that contains children and holds properties.
    ///
    /// Objects can be created by builtins, assignments, expressions and workbenches.
    Object(Object),

    /// A special node after which children will be nested as siblings.
    ///
    /// This node is removed after the children have been inserted.
    ChildrenPlaceholder,

    /// An affine transform.
    Transform(AffineTransform),

    /// A 2D geometry.
    Primitive2D(std::rc::Rc<Geometry2D>),

    /// A 3D geometry.
    Primitive3D(std::rc::Rc<Geometry3D>),

    /// An operation that generates a geometry from its children.
    Operation(std::rc::Rc<dyn Operation>),
}

impl Element {
    /// Get a property value from an [`Element`].
    ///
    /// Only objects can have properties.
    pub fn get_property_value(&self, id: &Identifier) -> Option<&Value> {
        match self {
            Self::Object(object) => object.get_property_value(id),
            _ => None,
        }
    }
}

/// The default [`ObjectNodeContent`] is an empty [`Object`].
impl Default for Element {
    fn default() -> Self {
        Element::Object(Object::default())
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
