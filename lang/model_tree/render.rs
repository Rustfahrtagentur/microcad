// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node methods and trait implementations for rendering.

use microcad_core::*;

use crate::model_tree::*;

impl ModelNode {
    /// Return output type.
    pub fn output_type(&self) -> ModelNodeOutputType {
        self.borrow().output().model_node_output_type()
    }

    /// Set the transformation matrices for this node and its children.
    pub fn set_matrix(&self, mat: Mat4) {
        let new_mat = {
            let mut b = self.borrow_mut();
            let new_mat = match b.element() {
                Element::Transform(affine_transform) => mat * affine_transform.mat3d(),
                _ => mat,
            };
            b.output_mut().matrix = new_mat;
            new_mat
        };

        self.children().for_each(|node| {
            node.set_matrix(new_mat);
        });
    }

    /// Set the resolution for this node.
    pub fn set_resolution(&self, resolution: RenderResolution) {
        let new_resolution = {
            let mut b = self.borrow_mut();
            let new_resolution = resolution * b.output().matrix;
            b.output_mut().resolution = new_resolution.clone();
            new_resolution
        };

        self.children().for_each(|node| {
            node.set_resolution(new_resolution.clone());
        });
    }

    /// Fetch output 2d geometries.
    ///
    /// Panics if the node does not contain any 2d geometry.
    pub fn fetch_output_geometries_2d(&self) -> Geometries2D {
        let b = self.borrow();
        match &b.output().geometry {
            ModelNodeGeometryOutput::Geometries2D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 2D geometry."),
        }
    }

    /// Fetch output 3d geometries.
    ///
    /// Panics if the node does not contain any 3d geometry.
    pub fn fetch_output_geometries_3d(&self) -> Geometries3D {
        let b = self.borrow();
        match &b.output().geometry {
            ModelNodeGeometryOutput::Geometries3D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 3D geometry."),
        }
    }

    /// Render the node.
    ///
    /// Rendering the node means that all geometry is calculated and stored
    /// in the respective model node output.
    /// This means after rendering, the rendered geometry can be retrieved via:
    /// * `fetch_output_geometries_2d()` for 2D geometries.
    /// * `fetch_output_geometries_3d()` for 3D geometries.
    pub fn render(&self) {
        fn render_geometries_2d(node: &ModelNode) -> Geometries2D {
            let b = node.borrow();
            match b.element() {
                Element::Primitive2D(geometry) => geometry.clone().into(),
                Element::Operation(operation) => operation.process_2d(node),
                _ => Geometries2D::default(),
            }
        }

        fn is_operation(node: &ModelNode) -> bool {
            let b = node.borrow();
            matches!(b.element(), Element::Operation(_))
        }

        match self.output_type() {
            ModelNodeOutputType::Geometry2D => {
                let geometries = render_geometries_2d(self);
                if !is_operation(self) {
                    self.children().for_each(|node| {
                        node.render();
                    });
                }

                let mut b = self.borrow_mut();
                b.output_mut().geometry = ModelNodeGeometryOutput::Geometries2D(geometries);
            }
            ModelNodeOutputType::Geometry3D => todo!(),
            output_type => println!("{output_type}"),
        }
    }
}

impl Operation for ModelNode {
    fn output_type(&self, node: &ModelNode) -> ModelNodeOutputType {
        node.output_type()
    }

    fn process_2d(&self, node: &ModelNode) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        let b = node.borrow();
        match b.element() {
            Element::Transform(_) | Element::Object(_) => {
                node.children()
                    .for_each(|node| geometries.append(node.process_2d(&node)));
            }
            Element::Primitive2D(geo) => {
                geometries.push(geo.clone());
                node.children()
                    .for_each(|node| geometries.append(node.process_2d(&node)));
            }
            Element::Operation(operation) => geometries.append(operation.process_2d(node)),
            _ => {}
        }

        geometries
    }
}

impl FetchBounds2D for ModelNode {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        let mut bounds = Bounds2D::default();

        self.descendants().for_each(|node| {
            let b = node.borrow();
            let output = b.output();
            if let ModelNodeGeometryOutput::Geometries2D(geometries) = &output.geometry {
                let mat = output.matrix_2d();
                let resolution = &output.resolution;
                bounds = bounds.clone().extend(
                    geometries
                        .fetch_bounds_2d()
                        .transformed_2d(resolution, &mat),
                );
            }
        });

        bounds
    }
}
