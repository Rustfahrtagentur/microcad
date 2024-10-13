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

use crate::render::{ModelNode, ModelNodeInner, Renderer2D};
use crate::Algorithm;
use geo::OpType;

fn into_group(node: ModelNode) -> Option<ModelNode> {
    node.first_child().and_then(|n| {
        if let ModelNodeInner::Group = *n.borrow() {
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
    fn process_2d(&self, renderer: &mut dyn Renderer2D, parent: ModelNode) -> crate::Result<ModelNode> {
        let mut geometries = Vec::new();

        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        group.children().try_for_each(|child| {
            let c = &*child.borrow();
            match c {
                ModelNodeInner::Primitive2D(renderable) => {
                    geometries.push(renderable.request_geometry(renderer)?)
                }
                ModelNodeInner::Geometry2D(g) => geometries.push(g.clone()),
                ModelNodeInner::Algorithm(algorithm) => {
                    let new_node = algorithm.process_2d(renderer, child.clone())?;
                    let c = &*new_node.borrow();
                    if let ModelNodeInner::Geometry2D(g) = c {
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

        Ok(ModelNode::new(ModelNodeInner::Geometry2D(result)))
    }

    fn process_3d(
        &self,
        renderer: &mut dyn crate::render::Renderer3D,
        parent: ModelNode,
    ) -> crate::Result<ModelNode> {
        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        let geometries: Vec<_> = group
            .children()
            .filter_map(|child| match &*child.borrow() {
                ModelNodeInner::Primitive3D(renderable) => renderable.request_geometry(renderer).ok(),
                ModelNodeInner::Geometry3D(g) => Some(g.clone()),
                ModelNodeInner::Algorithm(algorithm) => {
                    if let Ok(new_node) = algorithm.process_3d(renderer, child.clone()) {
                        if let ModelNodeInner::Geometry3D(g) = &*new_node.borrow() {
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
        Ok(ModelNode::new(ModelNodeInner::Geometry3D(
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
pub fn difference() -> ModelNode {
    ModelNode::new(ModelNodeInner::Algorithm(Box::new(BooleanOp::Difference)))
}

/// Short cut to generate a union operator node
pub fn union() -> ModelNode {
    ModelNode::new(ModelNodeInner::Algorithm(Box::new(BooleanOp::Union)))
}

/// Short cut to generate an intersection operator node
pub fn intersection() -> ModelNode {
    ModelNode::new(ModelNodeInner::Algorithm(Box::new(BooleanOp::Intersection)))
}

/// Short cut to generate a complement operator node
pub fn complement() -> ModelNode {
    ModelNode::new(ModelNodeInner::Algorithm(Box::new(BooleanOp::Complement)))
}

/// Short cut to generate boolean operator as binary operation with two nodes
pub fn binary_op(op: BooleanOp, lhs: ModelNode, rhs: ModelNode) -> ModelNode {
    assert!(lhs != rhs, "lhs and rhs must be distinct.");
    let root = ModelNode::new(ModelNodeInner::Algorithm(Box::new(op)));
    root.append(lhs);
    root.append(rhs);
    root
}
