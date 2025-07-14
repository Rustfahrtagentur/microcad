// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{model_tree::*, resolve::*, src_ref::*, syntax::*};

#[derive(Debug)]
struct Hull;

impl Operation for Hull {
    fn process_2d(&self, node: &ModelNode) -> microcad_core::Geometries2D {
        let mut geometries = Geometries2D::default();

        if let Some(node) = node.into_inner_object_node() {
            node.children().for_each(|node| {
                let b = node.borrow();
                match &b.element.value {
                    Element::Transform(affine_transform) => {
                        geometries.append(
                            node.process_2d(&node)
                                .transformed_2d(&b.output.resolution, &affine_transform.mat2d()),
                        );
                    }
                    _ => {
                        geometries.append(node.process_2d(&node));
                    }
                }
            });
        }

        geometries.hull(&node.borrow().output.resolution)
    }

    fn process_3d(
        &self,
        _node: &microcad_lang::model_tree::ModelNode,
    ) -> microcad_core::Geometries3D {
        std::todo!("Hull operation for 3D")
    }
}

/// Creates a symbol containing a difference operation.
pub fn hull() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("hull"), None, &|_, _, _| {
        Ok(ModelNodeBuilder::new_operation(Hull, SrcRef(None))
            .build()
            .into())
    })
}
