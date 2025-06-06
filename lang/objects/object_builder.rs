// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object builder module

use crate::{eval::*, objects::*};

/// Object builder to build up object nodes.
#[derive(Default)]
pub struct ObjectBuilder {
    object: Object,
    children: ObjectNodes,
}

impl ObjectBuilder {
    /// Initialize the properties by parameters and arguments.
    pub fn init_properties(
        &mut self,
        parameters: &ParameterValueList,
        arguments: &ArgumentMap,
    ) -> &mut Self {
        let mut props = ObjectProperties::default();

        for parameter in parameters.iter() {
            props.insert(
                parameter.id.clone(),
                match &parameter.default_value {
                    Some(value) => value.clone(),
                    None => arguments.get(&parameter.id).unwrap_or(&Value::None).clone(),
                },
            );
        }

        self.object.props = props;
        self
    }

    /// Add attributes to object.
    pub fn add_attributes(&mut self, attributes: MetaData) -> &mut Self {
        self.object.attributes = attributes;
        self
    }

    /// Append child nodes to this object node.
    pub fn append_children(&mut self, nodes: &mut ObjectNodes) -> &mut Self {
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

    /// Build the [ObjectNode].
    pub fn build_node(self) -> ModelNode {
        let node = ModelNode::new_object(self.object);
        for child in self.children.iter() {
            node.append(child.clone());
        }
        node
    }
}
