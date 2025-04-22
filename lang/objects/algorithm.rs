// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::{objects::*, rc_mut::*};
use microcad_core::*;

/// Algorithm trait
pub trait Algorithm: std::fmt::Debug {
    /// Calculates the 2D geometry for node
    fn render_into_geometry2d(
        &self,
        renderer: &mut Renderer2D,
        parent: ObjectNode,
    ) -> Option<Rc<geo2d::Geometry>> {
        self.process_2d(renderer, parent).ok().and_then(|new_node| {
            if let geo2d::NodeInner::Geometry(g) = &*new_node.borrow() {
                Some(g.clone())
            } else {
                None
            }
        })
    }

    /// Calculates the 2D geometry for node
    fn render_into_geometry3d(
        &self,
        renderer: &mut Renderer3D,
        parent: ObjectNode,
    ) -> Option<Rc<geo3d::Geometry>> {
        self.process_3d(renderer, parent).ok().and_then(|new_node| {
            if let geo3d::NodeInner::Geometry(g) = &*new_node.borrow() {
                Some(g.clone())
            } else {
                None
            }
        })
    }

    /// Calculates the 3D geometry for node
    fn process_geometry3d(
        &self,
        renderer: &mut Renderer3D,
        parent: ObjectNode,
    ) -> Option<Rc<geo3d::Geometry>> {
        self.process_3d(renderer, parent).ok().and_then(|new_node| {
            if let geo3d::NodeInner::Geometry(g) = &*new_node.borrow() {
                Some(g.clone())
            } else {
                None
            }
        })
    }

    /// Processes geometry for a 2d renderer and returns a geometry
    fn process_2d(
        &self,
        _renderer: &mut Renderer2D,
        _parent: ObjectNode,
    ) -> CoreResult<geo2d::Node> {
        unimplemented!()
    }

    /// Processes geometry for a 3d renderer and returns a geometry
    fn process_3d(
        &self,
        _renderer: &mut Renderer3D,
        _parent: ObjectNode,
    ) -> CoreResult<geo3d::Node> {
        unimplemented!()
    }
}
/// Short cut to generate a difference operator node
pub fn difference() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(Rc::new(BooleanOp::Difference)))
}

/// Short cut to generate a union operator node
pub fn union() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(Rc::new(BooleanOp::Union)))
}

/// Short cut to generate an intersection operator node
pub fn intersection() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(Rc::new(BooleanOp::Intersection)))
}

/// Short cut to generate a complement operator node
pub fn complement() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Algorithm(Rc::new(BooleanOp::Complement)))
}

/// Short cut to generate boolean operator as binary operation with two nodes
pub fn binary_op(op: BooleanOp, lhs: ObjectNode, rhs: ObjectNode) -> ObjectNode {
    assert!(lhs != rhs, "lhs and rhs must be distinct.");
    let root = ObjectNode::new(ObjectNodeInner::Algorithm(Rc::new(op)));
    let object = crate::objects::empty_object();
    object.append(lhs);
    object.append(rhs);

    root.append(object);
    root
}
