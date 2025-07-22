// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use crate::{GetPropertyValue, model::*, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// An element defines the entity of a [`Model`].
#[derive(Clone, IntoStaticStr, Debug)]
pub enum Element {
    /// An object that contains children and holds properties.
    ///
    /// Objects can be created by builtins, assignments, expressions and workbenches.
    Object(Object),

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

impl GetPropertyValue for Element {
    fn get_property_value(&self, id: &Identifier) -> Value {
        match self {
            Self::Object(object) => object.get_property_value(id),
            _ => Value::None,
        }
    }
}
