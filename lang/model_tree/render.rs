// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node methods and trait implementations for rendering.

use microcad_core::*;

use crate::model_tree::*;

impl ModelNodeInner {
    /// Return output type.
    pub fn output_type(&self) -> ModelNodeOutputType {
        self.output.model_node_output_type()
    }

    /// Set the transformation matrices for this node and its children.
    pub fn set_matrix(&mut self, mat: Mat4) {
        let new_mat = {
            let new_mat = match &self.element.value {
                Element::Transform(affine_transform) => mat * affine_transform.mat3d(),
                _ => mat,
            };
            self.output.matrix = new_mat;
            new_mat
        };

        self.children.iter().for_each(|node| {
            node.borrow_mut().set_matrix(new_mat);
        });
    }

    /// Set the resolution for this node.
    pub fn set_resolution(&mut self, resolution: RenderResolution) {
        let new_resolution = {
            let new_resolution = resolution * self.output.matrix;
            self.output.resolution = new_resolution.clone();
            new_resolution
        };

        self.children.iter().for_each(|node| {
            node.borrow_mut().set_resolution(new_resolution.clone());
        });
    }

    /// Fetch output 2d geometries.
    ///
    /// Panics if the node does not contain any 2d geometry.
    pub fn fetch_output_geometries_2d(&mut self) -> Geometries2D {
        match &self.output.geometry {
            ModelNodeGeometryOutput::Geometries2D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 2D geometry."),
        }
    }

    /// Fetch output 3d geometries.
    ///
    /// Panics if the node does not contain any 3d geometry.
    pub fn fetch_output_geometries_3d(&self) -> Geometries3D {
        match &self.output.geometry {
            ModelNodeGeometryOutput::Geometries3D(geometries) => geometries.clone(),
            _ => panic!("The node does not contain a 3D geometry."),
        }
    }
}

impl ModelNode {
    /// Render the node.
    ///
    /// Rendering the node means that all geometry is calculated and stored
    /// in the respective model node output.
    /// This means after rendering, the rendered geometry can be retrieved via:
    /// * `fetch_output_geometries_2d()` for 2D geometries.
    /// * `fetch_output_geometries_3d()` for 3D geometries.
    pub fn render(&self) {
        fn render_geometries_2d(node: &ModelNode) -> Geometries2D {
            match &node.borrow().element.value {
                Element::Primitive2D(geometry) => geometry.clone().into(),
                Element::Operation(operation) => operation.process_2d(node),
                _ => Geometries2D::default(),
            }
        }

        fn is_operation(node: &ModelNode) -> bool {
            matches!(&node.borrow().element.value, Element::Operation(_))
        }

        match self.output_type() {
            ModelNodeOutputType::Geometry2D => {
                let geometries = render_geometries_2d(self);
                if !is_operation(self) {
                    self.borrow().children.iter().for_each(|node| {
                        node.render();
                    });
                }

                self.borrow_mut().output.geometry =
                    ModelNodeGeometryOutput::Geometries2D(geometries);
            }
            ModelNodeOutputType::Geometry3D => todo!(),
            output_type => {
                panic!("Output type must have been determined at this point: {output_type}")
            }
        }
    }
}

impl Operation for ModelNode {
    fn process_2d(&self, node: &ModelNode) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        let node_ = &node.borrow();
        match &node_.element.value {
            Element::Transform(_) | Element::Object(_) => {
                node_
                    .children()
                    .for_each(|n| geometries.append(n.process_2d(n)));
            }
            Element::Primitive2D(geo) => {
                geometries.push(geo.clone());
                node_
                    .children()
                    .for_each(|n| geometries.append(n.process_2d(n)));
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
            let output = &node.borrow().output;
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
