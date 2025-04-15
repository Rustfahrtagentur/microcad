// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Boolean Op

pub use microcad_core::BooleanOp;

use crate::objects::*;
use microcad_core::*;

impl Algorithm for BooleanOp {
    fn process_2d(&self, renderer: &mut Renderer2D, parent: ObjectNode) -> CoreResult<geo2d::Node> {
        // all algorithm nodes are nested in a group

        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group(_) => {
                    BooleanOp::Union.render_into_geometry2d(renderer, child.clone())
                }
                ObjectNodeInner::Primitive2D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Transform(transform) => {
                    transform.render_into_geometry2d(renderer, child.clone())
                }
                ObjectNodeInner::Algorithm(algorithm) => {
                    algorithm.render_into_geometry2d(renderer, child.clone())
                }
                _ => None,
            })
            .collect();

        match geo2d::Geometry::boolean_op_multi(geometries, self) {
            Some(g) => Ok(geo2d::geometry(g)),
            None => Ok(geo2d::group()),
        }
    }

    fn process_3d(&self, renderer: &mut Renderer3D, parent: ObjectNode) -> CoreResult<geo3d::Node> {
        // all algorithm nodes are nested in a group

        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group(_) => {
                    BooleanOp::Union.process_geometry3d(renderer, child.clone())
                }
                ObjectNodeInner::Primitive3D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Transform(transform) => {
                    transform.render_into_geometry3d(renderer, child.clone())
                }
                ObjectNodeInner::Algorithm(algorithm) => {
                    algorithm.process_geometry3d(renderer, child.clone())
                }
                _ => None,
            })
            .collect();

        match geo3d::Geometry::boolean_op_multi(geometries, self) {
            Some(g) => Ok(geo3d::geometry(g)),
            None => Ok(geo3d::group()),
        }
    }
}
