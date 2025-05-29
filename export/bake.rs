// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bake module contains functions that convert object nodes into geometry nodes

use microcad_core::*;
use microcad_lang::*;

/// This function bakes the object node tree into a 2D geometry tree
pub fn bake2d(
    renderer: &mut Renderer2D,
    node: ObjectNode,
) -> core::result::Result<geo2d::Node, CoreError> {
    let node2d = {
        match *node.borrow() {
            ObjectNodeInner::Object(_) => geo2d::tree::group(),
            ObjectNodeInner::Export(_) => geo2d::tree::group(),
            ObjectNodeInner::Primitive2D(ref renderable) => {
                return Ok(geo2d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ));
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_2d(
                    renderer,
                    crate::objects::into_inner_object(node.clone()).unwrap_or(node.clone()),
                );
            }
            ObjectNodeInner::Transform(ref transform) => transform.into(),
            ObjectNodeInner::ChildrenNodeMarker => geo2d::tree::group(),
            _ => return Err(CoreError::NotImplemented),
        }
    };

    node.children().try_for_each(|child| {
        if let Ok(child) = bake2d(renderer, child) {
            node2d.append(child);
            Ok(())
        } else {
            Err(CoreError::NotImplemented)
        }
    })?;

    Ok(node2d)
}

/// This function bakes the object node tree into a 3D geometry tree
pub fn bake3d(
    renderer: &mut Renderer3D,
    node: ObjectNode,
) -> core::result::Result<geo3d::Node, CoreError> {
    let node3d = {
        match *node.borrow() {
            ObjectNodeInner::Object(_) => geo3d::tree::group(),
            ObjectNodeInner::Export(_) => geo3d::tree::group(),
            ObjectNodeInner::Primitive3D(ref renderable) => {
                return Ok(geo3d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ));
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_3d(
                    renderer,
                    crate::objects::into_inner_object(node.clone()).unwrap_or(node.clone()),
                );
            }
            ObjectNodeInner::Transform(ref transform) => transform.into(),
            ObjectNodeInner::ChildrenNodeMarker => geo3d::tree::group(),
            _ => return Err(CoreError::NotImplemented),
        }
    };

    node.children().try_for_each(|child| {
        if let Ok(child) = bake3d(renderer, child) {
            node3d.append(child);
            Ok(())
        } else {
            Err(CoreError::NotImplemented)
        }
    })?;

    Ok(node3d)
}
