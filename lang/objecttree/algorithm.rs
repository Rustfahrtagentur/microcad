// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::objecttree::{ObjectNode, ObjectNodeInner};
use microcad_core::*;

pub use microcad_core::BooleanOp;

/// Algorithm trait
pub trait Algorithm: std::fmt::Debug {
    /// Processes geometry for a 2d renderer and returns a geometry
    fn process_2d(&self, _renderer: &mut Renderer2D, _parent: ObjectNode) -> Result<geo2d::Node> {
        unimplemented!()
    }

    /// Processes geometry for a 3d renderer and returns a geometry
    fn process_3d(&self, _renderer: &mut Renderer3D, _parent: ObjectNode) -> Result<geo3d::Node> {
        unimplemented!()
    }
}

impl Algorithm for BooleanOp {
    fn process_2d(&self, renderer: &mut Renderer2D, parent: ObjectNode) -> Result<geo2d::Node> {
        // all algorithm nodes are nested in a group

        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group(_) => BooleanOp::Union
                    .process_2d(renderer, child.clone())
                    .ok()
                    .and_then(|new_node| {
                        if let geo2d::NodeInner::Geometry(g) = &*new_node.borrow() {
                            Some(g.clone())
                        } else {
                            None
                        }
                    }),
                ObjectNodeInner::Primitive2D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Algorithm(algorithm) => algorithm
                    .process_2d(renderer, child.clone())
                    .ok()
                    .and_then(|new_node| {
                        if let geo2d::NodeInner::Geometry(g) = &*new_node.borrow() {
                            Some(g.clone())
                        } else {
                            None
                        }
                    }),
                _ => None,
            })
            .collect();
        Ok(geo2d::geometry(geometries[1..].iter().fold(
            geometries[0].clone(),
            |acc, geo| {
                if let Some(r) = acc.boolean_op(geo.as_ref(), self) {
                    std::rc::Rc::new(r)
                } else {
                    acc
                }
            },
        )))
    }

    fn process_3d(&self, renderer: &mut Renderer3D, parent: ObjectNode) -> Result<geo3d::Node> {
        // all algorithm nodes are nested in a group

        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group(_) => BooleanOp::Union
                    .process_3d(renderer, child.clone())
                    .ok()
                    .and_then(|new_node| {
                        if let geo3d::NodeInner::Geometry(g) = &*new_node.borrow() {
                            Some(g.clone())
                        } else {
                            None
                        }
                    }),
                ObjectNodeInner::Primitive3D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Algorithm(algorithm) => algorithm
                    .process_3d(renderer, child.clone())
                    .ok()
                    .and_then(|new_node| {
                        if let geo3d::NodeInner::Geometry(g) = &*new_node.borrow() {
                            Some(g.clone())
                        } else {
                            None
                        }
                    }),
                _ => None,
            })
            .collect();
        Ok(geo3d::geometry(geometries[1..].iter().fold(
            geometries[0].clone(),
            |acc, geo| {
                if let Some(r) = acc.boolean_op(geo.as_ref(), self) {
                    std::rc::Rc::new(r)
                } else {
                    acc
                }
            },
        )))
    }
}

/// Short cut to generate a difference operator node
pub fn difference() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(std::rc::Rc::new(
        BooleanOp::Difference,
    )))
}

/// Short cut to generate a union operator node
pub fn union() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(std::rc::Rc::new(
        BooleanOp::Union,
    )))
}

/// Short cut to generate an intersection operator node
pub fn intersection() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(std::rc::Rc::new(
        BooleanOp::Intersection,
    )))
}

/// Short cut to generate a complement operator node
pub fn complement() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(std::rc::Rc::new(
        BooleanOp::Complement,
    )))
}

/// Short cut to generate boolean operator as binary operation with two nodes
pub fn binary_op(op: BooleanOp, lhs: ObjectNode, rhs: ObjectNode) -> ObjectNode {
    assert!(lhs != rhs, "lhs and rhs must be distinct.");
    let root = ObjectNode::new(ObjectNodeInner::Algorithm(std::rc::Rc::new(op)));
    root.append(lhs);
    root.append(rhs);
    root
}
