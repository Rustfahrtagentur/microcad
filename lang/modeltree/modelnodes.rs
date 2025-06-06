// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use microcad_core::BooleanOp;

use crate::{modeltree::*, src_ref::*};

/// Model node multiplicities.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ModelNodes(Vec<ModelNode>);

impl ModelNodes {
    /// Returns the first node if there is exactly one node in the list.
    pub fn single_node(&self) -> Option<ModelNode> {
        match self.0.len() {
            1 => self.0.first().cloned(),
            _ => None,
        }
    }

    /// Nest a Vec of node multiplicities
    ///
    /// * `node_stack`: A list of node lists.
    ///
    /// The reference to the first stack element will be returned.
    ///
    /// Assume, our node stack `Vec<Vec<Node>>` has for lists `a`, `b`, `c`, `d`:
    /// ```
    /// let nodes = vec![
    ///     vec![obj("a0"), obj("a1")],
    ///     vec![obj("b0")],
    ///     vec![obj("c0"), obj("c1"), obj("c2")],
    ///     vec![obj("d0")],
    /// ];
    /// ```
    ///
    /// This should result in following node multiplicity:
    /// a0
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    /// a1
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    pub fn from_node_stack(node_stack: &[ModelNodes]) -> Self {
        match node_stack.len() {
            0 => panic!("Node stack must not be empty"),
            1 => {}
            n => {
                (1..n)
                    .rev()
                    .map(|i| (&node_stack[i], &node_stack[i - 1]))
                    .for_each(|(prev_list, next_list)| {
                        // Insert a copy of each element `node` from `prev_list` as child to each element `new_parent` in `next_list`
                        next_list.iter().for_each(|new_parent_node| {
                            prev_list.iter().for_each(|node| {
                                node.detach();

                                // Handle children marker.
                                // If we have found a children marker node, use it's parent as new parent node.
                                let new_parent_node = match new_parent_node
                                    .find_children_placeholder()
                                {
                                    Some(children_marker) => {
                                        let parent =
                                            children_marker.parent().expect("Must have a parent");
                                        children_marker.detach(); // Remove children marker from tree
                                        parent
                                    }
                                    None => new_parent_node.clone(),
                                };

                                new_parent_node.append(node.make_deep_copy());
                            });
                        });
                    });
            }
        }

        node_stack[0].clone()
    }

    /// Return an algorithm node that unites all children.
    pub fn union(&self) -> ModelNode {
        match self.single_node() {
            Some(node) => node,
            None => {
                let union_node = ModelNode::new_transformation(BooleanOp::Union, SrcRef(None));
                union_node.append_children(self.clone())
            }
        }
    }

    /// Merge two lists of [`ObjectNode`] into one by concatenation.
    pub fn merge(lhs: ModelNodes, rhs: ModelNodes) -> Self {
        lhs.iter()
            .chain(rhs.iter())
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }

    /// Dump all nodes.
    pub fn dump(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.iter() {
            node.dump(f)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for ModelNodes {
    type Target = Vec<ModelNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModelNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<ModelNode>> for ModelNodes {
    fn from(value: Vec<ModelNode>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ModelNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.iter() {
            node.fmt(f)?;
        }
        Ok(())
    }
}

#[test]
fn node_nest() {
    use crate::syntax::*;

    fn obj(id: &str) -> ModelNode {
        ModelNode::new_empty_object(SrcRef(None)).set_id(Identifier::no_ref(id))
    }

    let nodes = vec![
        vec![obj("a0"), obj("a1")].into(),
        vec![obj("b0")].into(),
        vec![obj("c0"), obj("c1"), obj("c2")].into(),
        vec![obj("d0")].into(),
    ];

    // This should result in following node multiplicity:
    // a0
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    // a1
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    let nodes = ModelNodes::from_node_stack(&nodes);
    assert_eq!(nodes.len(), 2); // Contains a0 and a1 as root

    for node in nodes.iter() {
        node.descendants().for_each(|n| {
            log::trace!(
                "{}{}",
                "  ".repeat(n.depth()),
                match n.borrow().element() {
                    Element::Object(_) => node.id().expect("Id").clone(),
                    _ => panic!("Object with name expected"),
                }
            )
        });
    }
}
