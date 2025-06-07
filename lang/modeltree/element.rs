// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`ModelNode`]

use crate::{modeltree::*, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// Inner of a node
#[derive(Clone, IntoStaticStr, Debug)]
pub enum Element {
    /// An object that contains children and holds properties.
    ///
    /// Objects can be created by builtins, assignments, expressions and parts.
    Object(Object),

    /// A special node after which children will be nested as siblings.
    ///
    /// This node is removed after the children have been inserted.
    ChildrenPlaceholder,

    /// An affine transform.
    ///
    ///
    AffineTransform(AffineTransform),

    /// Generated 2D geometry.
    Primitive2D(std::rc::Rc<Primitive2D>),

    /// Generated 3D geometry.
    Primitive3D(std::rc::Rc<Primitive3D>),

    /// A transformation that generates a geometry.
    Transformation(std::rc::Rc<dyn Transformation>),
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
            Element::Transformation(transformation) => {
                write!(f, "({transformation:?})")
            }
            Element::Primitive2D(primitive2d) => {
                write!(f, "({primitive2d:?})")
            }
            _ => Ok(()),
        }
    }
}
