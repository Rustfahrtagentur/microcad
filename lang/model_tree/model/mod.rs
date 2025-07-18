// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model

mod model_builder;
mod model_inner;
mod models;
mod origin;

pub use model_builder::*;
pub use model_inner::*;
pub use models::*;
pub use origin::*;

use crate::{diag::WriteToFile, model_tree::*, rc::*, syntax::*, value::*, GetPropertyValue};
use derive_more::{Deref, DerefMut};
use microcad_core::*;

/// A reference counted, mutable [`Model`].
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Model(RcMut<ModelInner>);

impl Model {
    /// Create new model from inner.
    pub fn new(inner: RcMut<ModelInner>) -> Self {
        Self(inner)
    }

    /// Calculate Depth of the model.
    pub fn depth(&self) -> usize {
        self.parents().count()
    }

    /// Make a deep copy if this model.
    /// TODO: isn't this a Clone?
    pub fn make_deep_copy(&self) -> Self {
        let copy = Self(RcMut::new(self.0.borrow().clone_content()));
        for child in self.borrow().children.iter() {
            copy.append(child.make_deep_copy());
        }
        copy
    }

    /// Return address of this model.
    pub fn addr(&self) -> usize {
        self.0.as_ptr().addr()
    }

    /// Check if `other` is and `self` have the same address.
    pub fn is_same_as(&self, other: &Model) -> bool {
        self.addr() == other.addr()
    }

    /// Remove child from this model.
    pub fn remove_child(&self, child: &Model) {
        let mut s = self.0.borrow_mut();
        s.children.retain(|model| !model.is_same_as(child));
    }

    /// Detaches a model from its parent. Children are not affected.
    pub fn detach(&self) {
        match self.0.borrow_mut().parent {
            Some(ref mut parent) => {
                parent.remove_child(self);
            }
            None => return,
        }

        self.0.borrow_mut().parent = None;
    }

    /// Append a single model as child.
    ///
    /// Also tries to set the output type if it has not been determined yet.
    pub fn append(&self, model: Model) -> Model {
        model.borrow_mut().parent = Some(self.clone());

        let mut self_ = self.0.borrow_mut();
        self_.children.push(model.clone());

        model
    }

    /// Append multiple models as children.
    ///
    /// Return self.
    pub fn append_children(&self, models: Models) -> Self {
        for model in models.iter() {
            self.append(model.clone());
        }
        self.clone()
    }

    /// Short cut to generate boolean operator as binary operation with two models.
    pub fn boolean_op(self, op: BooleanOp, other: Model) -> Model {
        assert!(self != other, "lhs and rhs must be distinct.");
        Models::from(vec![self.clone(), other]).boolean_op(op)
    }

    /// Find children model placeholder in model descendants.
    pub fn find_children_placeholder(&self) -> Option<Model> {
        self.descendants().find(|n| {
            n.borrow().id.is_none()
                && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
        })
    }

    /// Find the original source file of this model
    pub fn find_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.ancestors()
            .find_map(|model| model.borrow().origin.source_file.clone())
    }

    /// Test if the model has this specific source file.
    pub fn has_source_file(&self, source_file: &std::rc::Rc<SourceFile>) -> bool {
        match (source_file.as_ref(), self.find_source_file()) {
            (a, Some(b)) => a.hash == b.hash,
            _ => false,
        }
    }

    /// Return inner model if we are in an [`Object`].
    pub fn into_inner_object_model(&self) -> Option<Model> {
        self.borrow().children.iter().next().and_then(|n| {
            if let Element::Object(_) = n.0.borrow().element.value {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// A [`Model`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
    pub fn signature(&self) -> String {
        let self_ = self.borrow();

        format!(
            "{id}{element_type}{origin} -> {output_type}",
            id = match &self_.id {
                Some(id) => format!("{id}: "),
                None => String::new(),
            },
            element_type = self_.element,
            origin = match self_.origin.creator {
                Some(_) => format!(" = {origin}", origin = self_.origin),
                None => String::new(),
            },
            output_type = self.final_output_type()
        )
    }
}

/// Iterator methods.
impl Model {
    /// Returns an iterator of models to this model and its unnamed descendants, in tree order.
    ///
    /// Includes the current model.
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    /// Returns an iterator of models that belong to the same source file as this one
    pub fn source_file_descendants(&self) -> SourceFileDescendants {
        SourceFileDescendants::new(self.clone())
    }

    /// Parents iterator.
    pub fn parents(&self) -> Parents {
        Parents::new(self.clone())
    }

    /// Ancestors iterator.
    pub fn ancestors(&self) -> Ancestors {
        Ancestors::new(self.clone())
    }
}

impl GetPropertyValue for Model {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.borrow().element.get_property_value(id)
    }
}

impl GetAttribute for Model {
    fn get_attribute(&self, id: &Identifier) -> Option<crate::model_tree::Attribute> {
        self.borrow().attributes.get_attribute(id)
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

/// Prints a [`Model`].
///
/// A [`Model`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
/// The exemplary output will look like this:
///
/// ```custom
/// id: Object:
///     Object = std::geo2d::circle(radius = 3.0mm) -> Geometry2D:
///         Primitive = __builtin::geo2d::circle(radius = 3.0) -> Geometry2D`
/// ```
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.depth() * 2;
        writeln!(f, "{:depth$}{signature}", "", signature = self.signature())?;
        for child in self.borrow().children.iter() {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

impl WriteToFile for Model {}
