// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model

use crate::{model::*, resolve::*, src_ref::*, syntax::*};

/// The actual model contents
#[derive(custom_debug::Debug, Default)]
pub struct ModelInner {
    /// Optional id.
    ///
    /// The id is set when the model was created by an assignment: `a = Cube(50mm)`.
    pub id: Option<Identifier>,
    /// Parent object.
    #[debug(skip)]
    pub parent: Option<Model>,
    /// Children of the model.
    pub children: Models,
    /// Element of the model with [SrcRef].
    pub element: Element,
    /// Attributes used for export.
    pub attributes: Attributes,
    /// The symbol (e.g. [`WorkbenchDefinition`]) that created this [`Model`].
    pub origin: Origin,
    /// The output type of the this model.
    pub output: ModelOutput,
}

impl ModelInner {
    /// Create a new [`ModelInner`] with a specific element.
    pub fn new(element: Element) -> Self {
        Self {
            element,
            ..Default::default()
        }
    }

    /// Clone only the content of this model without children and parent.
    pub fn clone_content(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: None,
            element: self.element.clone(),
            attributes: self.attributes.clone(),
            origin: self.origin.clone(),
            output: self.output.clone(),
            ..Default::default()
        }
    }

    /// Return iterator of children.s
    pub fn children(&self) -> std::slice::Iter<'_, Model> {
        self.children.iter()
    }

    /// Return if ,model has no children.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Set the information about the creator of this model.
    ///
    /// This function is called after the resulting models of a call of a part
    /// have been retrieved.   
    pub(crate) fn set_creator(&mut self, creator: Symbol, call_src_ref: SrcRef) {
        self.origin.set_creator(creator);
        self.origin.call_src_ref = call_src_ref;
    }
}

impl PropertiesAccess for ModelInner {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        self.element.get_property(id)
    }

    fn add_properties(&mut self, props: Properties) {
        self.element.add_properties(props);
    }
}
