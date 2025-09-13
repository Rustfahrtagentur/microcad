// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

pub mod attribute;
pub mod builder;
pub mod creator;
pub mod element;
mod inner;
pub mod iter;
pub mod models;
pub mod operation;
pub mod output_type;
pub mod properties;
pub mod workpiece;

pub use attribute::*;
pub use builder::*;
pub use creator::*;
pub use element::*;
pub use inner::*;
pub use iter::*;
pub use models::*;
pub use operation::*;
pub use output_type::*;
pub use properties::*;
pub use workpiece::*;

use derive_more::{Deref, DerefMut};

use microcad_core::BooleanOp;

use crate::{
    diag::WriteToFile, rc::RcMut, src_ref::SrcReferrer, syntax::Identifier, tree_display::*,
    value::Value,
};

/// A reference counted, mutable [`Model`].
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Model(RcMut<ModelInner>);

impl Model {
    /// Create new model from inner.
    pub fn new(inner: RcMut<ModelInner>) -> Self {
        Self(inner)
    }

    /// Calculate depth of the model.
    pub fn depth(&self) -> usize {
        self.parents().count()
    }

    /// Check if a model contains an operation element.
    pub fn is_operation(&self) -> bool {
        self.borrow().element.is_operation()
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

    /// Replace each input placeholder with copies of `input_model`.
    pub fn replace_input_placeholders(&self, input_model: &Model) -> Self {
        self.descendants().for_each(|model| {
            let mut model_ = model.borrow_mut();
            if model_.id.is_none() && matches!(model_.element.value, Element::InputPlaceholder) {
                let mut input_model_ = input_model.borrow_mut();
                input_model_.parent = Some(self.clone());
                *model_ = input_model_.clone_content();
                model_.children = input_model_.children.clone();
            }
        });
        self.clone()
    }

    /// Deduce output type from children and set it and return it.
    pub fn deduce_output_type(&self) -> OutputType {
        let self_ = self.borrow();
        let mut output_type = self_.element.output_type();
        if output_type == OutputType::NotDetermined {
            let children = &self_.children;
            output_type = children.deduce_output_type();
        }

        output_type
    }

    /// Return inner group if this model only contains a group as single child.
    ///
    /// This function is used when we evaluate operations like `subtract() {}` or `hull() {}`.
    /// When evaluating these operations, we want to iterate over the group's children.
    pub fn into_group(&self) -> Option<Model> {
        let children = &self.borrow().children;
        if children.len() != 1 {
            return None;
        }

        children.first().and_then(|n| {
            if let Element::Group = *n.0.borrow().element {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// A [`Model`] signature has the form `[id: ]ElementType[ = origin][ -> result_type]`.
    pub fn signature(&self) -> String {
        format!(
            "{id}{element}{is_root} ->",
            id = match &self.borrow().id {
                Some(id) => format!("{id}: "),
                None => String::new(),
            },
            element = *self.borrow().element,
            is_root = if self.parents().next().is_some() {
                ""
            } else {
                " (root)"
            }
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

    /// Get a property from this model.
    pub fn get_property(&self, id: &Identifier) -> Option<Value> {
        self.borrow().element.get_property(id).cloned()
    }

    /// Add a new property to the model.
    pub fn add_property(&mut self, id: Identifier, value: Value) {
        self.borrow_mut()
            .element
            .add_properties([(id, value)].into_iter().collect())
    }
}

impl AttributesAccess for Model {
    fn get_attributes_by_id(&self, id: &Identifier) -> Vec<Attribute> {
        self.borrow().attributes.get_attributes_by_id(id)
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

impl SrcReferrer for Model {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.borrow().src_ref()
    }
}

/// Prints a [`Model`].
///
/// A [`Model`] signature has the form `[id: ]ElementType[ = origin][ -> result_type]`.
/// The exemplary output will look like this:
///
/// ```custom
/// id: Object:
///     Object = std::geo2d::Circle(radius = 3.0mm) -> Geometry2D:
///         Primitive = __builtin::geo2d::Circle(radius = 3.0) -> Geometry2D`
/// ```
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{signature}",
            signature = crate::shorten!(self.signature())
        )
    }
}

impl TreeDisplay for Model {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter,
        mut tree_state: TreeState,
    ) -> std::fmt::Result {
        let signature = crate::shorten!(self.signature(), tree_state.shorten);
        let self_ = self.borrow();
        if let Some(output) = &self_.output {
            writeln!(f, "{:tree_state$}{signature} {output}", "",)?;
        } else {
            writeln!(f, "{:tree_state$}{signature}", "",)?;
        }
        tree_state.indent();
        if let Some(props) = self_.get_properties() {
            props.tree_print(f, tree_state)?;
        }
        self_.attributes.tree_print(f, tree_state)?;
        self_.children.tree_print(f, tree_state)
    }
}

impl WriteToFile for Model {}
