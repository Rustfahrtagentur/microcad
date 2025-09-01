// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

mod cache;

use std::rc::Rc;

pub use cache::*;

use cgmath::SquareMatrix;
use microcad_core::*;
use thiserror::Error;

use crate::{eval::BuiltinWorkbenchKind, model::*, value::ValueError};

/// An error that occurred during rendering.
#[derive(Debug, Error)]
pub enum RenderError {
    /// Value error
    #[error("Value Error: {0}")]
    ValueError(#[from] ValueError),
}

/// A result from rendering a model.
pub type RenderResult<T> = Result<T, RenderError>;

impl Model {
    /// Return output type.
    pub fn final_output_type(&self) -> OutputType {
        self.deduce_output_type()
    }

    /// Deduce output type from children and set it and return it.
    pub fn deduce_output_type(&self) -> OutputType {
        let self_ = self.borrow();
        let mut output_type = self_.element.output_type();
        if output_type == OutputType::NotDetermined {
            let children = &self_.children;
            output_type = children.deduce_output_type();
        }

        output_type
    }

    /// Fetch output 2d geometries.
    ///
    /// Panics if the model does not contain any 2d geometry.
    pub fn fetch_output_geometry_2d(&self) -> Option<Rc<Geometry2D>> {
        todo!()
    }

    /// Fetch output 3d geometries.
    ///
    /// Panics if the model does not contain any 3d geometry.
    pub fn fetch_output_geometry_3d(&self) -> Option<Rc<Geometry3D>> {
        todo!()
    }

    /// Render geometries in 2D.
    pub fn render_geometry_2d(&self, cache: &mut RenderCache) -> RenderResult<Geometries2D> {
        todo!()
    }

    /// Render geometries in 3D.
    pub fn render_geometry_3d(&self, _cache: &mut RenderCache) -> RenderResult<Geometries3D> {
        todo!()
    }

    /// Render the model.
    ///
    /// Rendering the model means that all geometry is calculated and stored
    /// in the in the render cache.
    pub fn render(
        &self,
        resolution: RenderResolution,
        _cache: &mut RenderCache,
    ) -> RenderResult<()> {
        pub fn create_render_output(model: &Model) -> RenderResult<()> {
            let output = RenderOutput::new(model)?;
            {
                let mut model_ = model.borrow_mut();
                model_.output = output;
            };

            model
                .borrow()
                .children
                .iter()
                .try_for_each(|model| create_render_output(model))
        }

        pub fn set_world_matrix(model: &Model, matrix: Mat4) -> RenderResult<()> {
            let world_matrix = {
                let mut model_ = model.borrow_mut();
                let output = model_.output.as_mut().expect("Output");
                let world_matrix = matrix * output.local_matrix();
                output.set_world_matrix(world_matrix);
                world_matrix
            };

            model
                .borrow()
                .children
                .iter()
                .try_for_each(|model| set_world_matrix(model, world_matrix))
        }

        /// Set the resolution for this model.
        pub fn set_resolution(model: &Model, resolution: RenderResolution) {
            let new_resolution = {
                let mut model_ = model.borrow_mut();
                let output = model_.output.as_mut().expect("Output");

                let resolution = resolution * output.world_matrix();
                output.set_resolution(resolution.clone());
                resolution
            };

            model.borrow().children.iter().for_each(|model| {
                set_resolution(model, new_resolution.clone());
            });
        }

        pub fn render_geometry(model: &Model) {
            let mut model_ = model.borrow_mut();
            // Bottom up search
            model_.children.iter().for_each(|model| {
                render_geometry(model);
            });

            let mut output = model_.output.as_mut().expect("Output");

            match output {
                RenderOutput::Geometry2D {
                    local_matrix,
                    world_matrix,
                    resolution,
                    geometry,
                } => {
                    todo!();
                    //*geometry = Some(element.render_2d());
                }
                RenderOutput::Geometry3D {
                    local_matrix,
                    world_matrix,
                    resolution,
                    geometry,
                } => todo!(),
            }
        }

        // Create specific render output with local matrix.
        create_render_output(self);

        // Calculate the world matrix.
        set_world_matrix(self, Mat4::identity());

        // Calculate the resolution for the model.
        set_resolution(self, resolution);

        eprintln!("Tree:\n{}", FormatTree(self));

        // Now we can render the geometry

        todo!("Implement render algorithm")
    }
}
