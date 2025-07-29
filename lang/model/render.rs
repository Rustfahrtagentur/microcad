// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

use cgmath::SquareMatrix;
use microcad_core::*;

use crate::model::*;

impl Model {
    /// Return output type.
    pub fn final_output_type(&self) -> OutputType {
        self.borrow().output.output_type()
    }

    /// Deduce output type from children and set it and return it.
    pub fn deduce_output_type(&self) -> OutputType {
        let mut self_ = self.borrow_mut();
        let mut output_type = match &*self_.element {
            Element::Group
            | Element::Workpiece(_, _)
            | Element::ChildrenPlaceholder
            | Element::Transform(_) => OutputType::NotDetermined,
            Element::Primitive2D(_) => OutputType::Geometry2D,
            Element::Primitive3D(_) => OutputType::Geometry3D,
            Element::Operation(operation) => operation.output_type(),
        };
        if output_type == OutputType::NotDetermined {
            let children = &self_.children;
            output_type = children.deduce_output_type();
        }

        self_.output = ModelOutput::new(output_type);

        output_type
    }

    /// Set the transformation matrices for this model and its children.
    pub fn set_matrix(&self, mat: Mat4) {
        let world_matrix = {
            let mut self_ = self.borrow_mut();
            let local_matrix = match &self_.element.value {
                Element::Transform(affine_transform) => affine_transform.mat3d(),
                _ => Mat4::identity(),
            };
            self_.output.world_matrix = mat * local_matrix;
            self_.output.local_matrix = local_matrix;
            self_.output.world_matrix
        };

        self.borrow().children.iter().for_each(|model| {
            model.set_matrix(world_matrix);
        });
    }

    /// Set the resolution for this model.
    pub fn set_resolution(&self, resolution: RenderResolution) {
        let new_resolution = {
            let mut self_ = self.borrow_mut();
            let new_resolution = resolution * self_.output.world_matrix;
            self_.output.resolution = new_resolution.clone();
            new_resolution
        };

        self.borrow().children.iter().for_each(|model| {
            model.set_resolution(new_resolution.clone());
        });
    }

    /// Fetch output 2d geometries.
    ///
    /// Panics if the model does not contain any 2d geometry.
    pub fn fetch_output_geometries_2d(&self) -> Geometries2D {
        match &self.borrow().output.geometry {
            GeometryOutput::Geometries2D(geometries) => geometries.clone(),
            _ => panic!("The model does not contain a 2D geometry."),
        }
    }

    /// Fetch output 3d geometries.
    ///
    /// Panics if the model does not contain any 3d geometry.
    pub fn fetch_output_geometries_3d(&self) -> Geometries3D {
        match &self.borrow().output.geometry {
            GeometryOutput::Geometries3D(geometries) => geometries.clone(),
            _ => panic!("The model does not contain a 3D geometry."),
        }
    }

    /// Render the model.
    ///
    /// Rendering the model means that all geometry is calculated and stored
    /// in the respective model output.
    /// This means after rendering, the rendered geometry can be retrieved via:
    /// * `fetch_output_geometries_2d()` for 2D geometries.
    /// * `fetch_output_geometries_3d()` for 3D geometries.
    pub fn render(&self) {
        fn render_geometries_2d(model: &Model) -> Geometries2D {
            match &model.borrow().element.value {
                Element::Primitive2D(geometry) => geometry.clone().into(),
                Element::Operation(operation) => operation.process_2d(model),
                _ => Geometries2D::default(),
            }
        }

        fn is_operation(model: &Model) -> bool {
            matches!(&model.borrow().element.value, Element::Operation(_))
        }

        match self.final_output_type() {
            OutputType::Geometry2D => {
                let geometries = render_geometries_2d(self);
                if !is_operation(self) {
                    self.borrow().children.iter().for_each(|model| {
                        model.render();
                    });
                }

                self.borrow_mut().output.geometry = GeometryOutput::Geometries2D(geometries);
            }
            OutputType::Geometry3D => todo!(),
            output_type => {
                panic!("Output type must have been determined at this point: {output_type}\n{self}")
            }
        }
    }
}

impl Operation for Model {
    fn process_2d(&self, model: &Model) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        let model_ = &model.borrow();
        match &model_.element.value {
            Element::Group | Element::Transform(_) | Element::Workpiece(_, _) => {
                model_
                    .children()
                    .for_each(|n| geometries.append(n.process_2d(n)));
            }
            Element::Primitive2D(geo) => {
                geometries.push(geo.clone());
                model_
                    .children()
                    .for_each(|n| geometries.append(n.process_2d(n)));
            }
            Element::Operation(operation) => geometries.append(operation.process_2d(model)),
            _ => {}
        }

        geometries.transformed_2d(&model_.output.resolution, &model_.output.local_matrix_2d())
    }
}

impl FetchBounds2D for Model {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        let mut bounds = Bounds2D::default();

        self.descendants().for_each(|model| {
            let output = &model.borrow().output;
            if let GeometryOutput::Geometries2D(geometries) = &output.geometry {
                let mat = output.world_matrix_2d();
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
