// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object builder module

use crate::{eval::*, model_tree::*, src_ref::*, syntax::*, value::*};

/// Object builder to build up a [ModelNode] with an object element.
#[derive(Default)]
pub struct ObjectBuilder {
    /// The object element to be built.
    object: Object,

    /// Children to be placed in the node.
    children: ModelNodes,

    /// The [`SrcRef`] attached to the object element.
    src_ref: SrcRef,
}

impl ObjectBuilder {
    /// Create new [ObjectBuilder] with a [SrcRef].
    pub fn new(src_ref: SrcRef) -> Self {
        Self {
            src_ref,
            ..Default::default()
        }
    }

    /// Create a new object with the properties by parameters and arguments.
    pub fn new_object_with_properties(
        src_ref: SrcRef,
        parameters: &ParameterValueList,
        arguments: &ArgumentMap,
    ) -> Self {
        let mut object_builder = ObjectBuilder::new(src_ref);
        let mut props = ObjectProperties::default();

        for parameter in parameters.iter() {
            props.insert(
                parameter.id.clone(),
                match &parameter.default_value {
                    Some(value) => value.clone(),
                    None => arguments
                        .get_value(&parameter.id)
                        .unwrap_or(&Value::None)
                        .clone(),
                },
            );
        }

        object_builder.object.props = props;
        object_builder
    }

    /// Append child nodes to this object node.
    pub fn append_children(&mut self, nodes: &mut ModelNodes) -> &mut Self {
        (*self.children).append(nodes);
        self
    }

    /// Set property value for object.
    pub fn set_property(&mut self, id: Identifier, value: Value) -> &mut Self {
        self.object.props.insert(id, value);
        self
    }

    /// Return true if the object has a property with `id`.
    pub fn has_property(&mut self, id: &Identifier) -> bool {
        self.object.props.contains_key(id)
    }

    /// Add all object properties to scope
    pub fn properties_to_scope(&mut self, context: &mut Context) -> EvalResult<()> {
        if self.object.props.is_valid() {
            for (id, value) in self.object.props.iter() {
                context.set_local_value(id.clone(), value.clone())?;
            }

            Ok(())
        } else {
            Err(EvalError::UninitializedProperties(
                self.object.props.get_ids_of_uninitialized().into(),
            ))
        }
    }

    /// Build the [`ModelNode`].
    pub fn build_node(self) -> ModelNode {
        let node = ModelNode::new_object(self.object, self.src_ref);
        node.append_children(self.children);
        node
    }
}
