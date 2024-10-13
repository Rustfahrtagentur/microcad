// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Boolean operations

#[derive(Debug)]
/// Boolean operations
pub enum BooleanOp {
    /// Computes the union R = P ∪ Q
    Union,
    /// computes the difference R = P ∖ Q
    Difference,
    /// computes the complement R=P̅
    Complement,
    /// computes the intersection R = P ∩ Q
    Intersection,
}

use crate::render::{Node, NodeInner, Renderer2D};
use crate::Algorithm;
use geo::OpType;

fn into_group(node: Node) -> Option<Node> {
    node.first_child().and_then(|n| {
        if let NodeInner::Group = *n.borrow() {
            Some(n.clone())
        } else {
            None
        }
    })
}

impl From<BooleanOp> for OpType {
    fn from(op: BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Complement => OpType::Xor,
        }
    }
}

impl From<&BooleanOp> for OpType {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Complement => OpType::Xor,
        }
    }
}

impl Algorithm for BooleanOp {
    fn process_2d(&self, renderer: &mut dyn Renderer2D, parent: Node) -> crate::Result<Node> {
        let mut geometries = Vec::new();

        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        group.children().try_for_each(|child| {
            let c = &*child.borrow();
            match c {
                NodeInner::Primitive2D(renderable) => {
                    geometries.push(renderable.request_geometry(renderer)?)
                }
                NodeInner::Geometry2D(g) => geometries.push(g.clone()),
                NodeInner::Algorithm(algorithm) => {
                    let new_node = algorithm.process_2d(renderer, child.clone())?;
                    let c = &*new_node.borrow();
                    if let NodeInner::Geometry2D(g) = c {
                        geometries.push(g.clone())
                    }
                }
                _ => (),
            };

            Ok::<(), crate::CoreError>(())
        })?;

        let mut result = geometries[0].clone();

        geometries[1..].iter().for_each(|geo| {
            if let Some(r) = result.boolean_op(geo.as_ref(), self) {
                result = std::rc::Rc::new(r)
            }
        });

        Ok(Node::new(NodeInner::Geometry2D(result)))
    }

    fn process_3d(
        &self,
        renderer: &mut dyn crate::render::Renderer3D,
        parent: Node,
    ) -> crate::Result<Node> {
        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        let geometries: Vec<_> = group
            .children()
            .filter_map(|child| match &*child.borrow() {
                NodeInner::Primitive3D(renderable) => renderable.request_geometry(renderer).ok(),
                NodeInner::Geometry3D(g) => Some(g.clone()),
                NodeInner::Algorithm(algorithm) => {
                    if let Ok(new_node) = algorithm.process_3d(renderer, child.clone()) {
                        if let NodeInner::Geometry3D(g) = &*new_node.borrow() {
                            Some(g.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        Ok(Node::new(NodeInner::Geometry3D(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(geo.as_ref(), self) {
                        std::rc::Rc::new(r)
                    } else {
                        acc
                    }
                }),
        )))
    }
}

/// Short cut to generate a difference operator node
pub fn difference() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Difference)))
}

/// Short cut to generate a union operator node
pub fn union() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Union)))
}

/// Short cut to generate an intersection operator node
pub fn intersection() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Intersection)))
}

/// Short cut to generate a complement operator node
pub fn complement() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Complement)))
}

/// Short cut to generate boolean operator as binary operation with two nodes
pub fn binary_op(op: BooleanOp, lhs: Node, rhs: Node) -> Node {
    assert!(lhs != rhs, "lhs and rhs must be distinct.");
    let root = Node::new(NodeInner::Algorithm(Box::new(op)));
    root.append(lhs);
    root.append(rhs);
    root
}
