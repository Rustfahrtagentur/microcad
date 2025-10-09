// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

mod cache;
mod context;
mod output;

use std::rc::Rc;

pub use cache::*;
pub use context::*;
pub use output::*;

use cgmath::SquareMatrix;
use microcad_core::*;
use thiserror::Error;

use crate::{
    builtin::{BuiltinWorkbenchKind, BuiltinWorkpiece, BuiltinWorkpieceOutput},
    model::*,
    tree_display::FormatTree,
};

/// An error that occurred during rendering.
#[derive(Debug, Error)]
pub enum RenderError {
    /// Invalid output type.
    #[error("Invalid output type: {0}")]
    InvalidOutputType(OutputType),

    /// Nothing to render.
    #[error("Nothing to render")]
    NothingToRender,
}

/// A result from rendering a model.
pub type RenderResult<T> = Result<T, RenderError>;

/// The render trait.
pub trait Render<T> {
    /// Render method.
    fn render(&self, context: &mut RenderContext) -> RenderResult<T>;
}

impl Element {
    /// Fetch the local matrix
    pub fn get_affine_transform(&self) -> RenderResult<Option<AffineTransform>> {
        match &self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Transform => match builtin_workpiece.call()? {
                    BuiltinWorkpieceOutput::Transform(affine_transform) => {
                        Ok(Some(affine_transform))
                    }
                    _ => unreachable!(),
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

impl ModelInner {
    /// Get render resolution.
    pub fn resolution(&self) -> RenderResolution {
        let output = self.output.as_ref().expect("Some render output.");

        match output {
            RenderOutput::Geometry2D { resolution, .. }
            | RenderOutput::Geometry3D { resolution, .. } => {
                resolution.as_ref().expect("Some resolution.").clone()
            }
        }
    }
}

impl Model {
    /// Pre render the model.
    ///
    /// Rendering the model means that all geometry is calculated and stored
    /// in the in the render cache.
    pub fn prerender(&self, resolution: RenderResolution) -> RenderResult<()> {
        pub fn create_render_output(model: &Model) -> RenderResult<()> {
            let output = RenderOutput::new(model)?;
            {
                let mut model_ = model.borrow_mut();
                model_.output = Some(output);
            };

            model
                .borrow()
                .children
                .iter()
                .try_for_each(create_render_output)
        }

        pub fn set_world_matrix(model: &Model, matrix: Mat4) -> RenderResult<()> {
            let world_matrix = {
                let mut model_ = model.borrow_mut();
                let output = model_.output.as_mut().expect("Output");
                let world_matrix = matrix * output.local_matrix().unwrap_or(Mat4::identity());
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

                let resolution = resolution * output.local_matrix().unwrap_or(Mat4::identity());
                output.set_resolution(resolution.clone());
                resolution
            };

            model.borrow().children.iter().for_each(|model| {
                set_resolution(model, new_resolution.clone());
            });
        }

        // Create specific render output with local matrix.
        create_render_output(self)?;

        // Calculate the world matrix.
        set_world_matrix(self, Mat4::identity())?;

        // Calculate the resolution for the model.
        set_resolution(self, resolution);

        log::trace!("Finished prerender:\n{}", FormatTree(self));

        Ok(())
    }
}

impl FetchBounds2D for Model {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        let self_ = self.borrow();
        match self_.output() {
            RenderOutput::Geometry2D { geometry, .. } => match geometry {
                Some(geometry) => geometry.fetch_bounds_2d(),
                None => Bounds2D::default(),
            },
            RenderOutput::Geometry3D { .. } => Bounds2D::default(),
        }
    }
}

/// This implementation renders a [`Geometry2D`] out of a [`Model`].
///
/// Notes:
/// * The impl attaches the output geometry to the model's render output.
/// * It is assumed the model has been pre-rendered.
impl Render<Geometry2DOutput> for Model {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.with_model(self.clone(), |context| {
            let model = context.model();
            let geometry = {
                let model_ = model.borrow();
                let output = model.deduce_output_type();
                match output {
                    OutputType::Geometry2D => {
                        match model_.element() {
                            // A group geometry will render the child geometry
                            Element::BuiltinWorkpiece(builtin_workpiece) => {
                                builtin_workpiece.render(context)
                            }
                            _ => model_.children.render(context),
                        }
                    }
                    _ => Ok(None),
                }
            }?;

            self.borrow_mut()
                .output_mut()
                .set_geometry_2d(geometry.clone());
            Ok(geometry)
        })
    }
}

/// This implementation renders a [`Geometry3D`] out of a [`Model`].
///
/// Notes:
/// * The impl attaches the output geometry to the model's render output.
/// * It is assumed the model has been pre-rendered.
impl Render<Geometry3DOutput> for Model {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.with_model(self.clone(), |context| {
            let model = context.model();
            let geometry = {
                let model_ = model.borrow();
                let output = model.deduce_output_type();
                match output {
                    OutputType::Geometry3D => {
                        match model_.element() {
                            // A group geometry will render the child geometry
                            Element::BuiltinWorkpiece(builtin_workpiece) => {
                                builtin_workpiece.render(context)
                            }
                            _ => model_.children.render(context),
                        }
                    }
                    _ => Ok(None),
                }
            }?;

            self.borrow_mut()
                .output_mut()
                .set_geometry_3d(geometry.clone());
            Ok(geometry)
        })
    }
}

impl Render<Model> for Model {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Model> {
        match self.deduce_output_type() {
            OutputType::Geometry2D => {
                let _: Geometry2DOutput = self.render(context)?;
            }
            OutputType::Geometry3D => {
                let _: Geometry3DOutput = self.render(context)?;
            }
            output_type => {
                log::warn!("Nothing to render: {output_type}");
            }
        }
        log::trace!("Finished render:\n{}", FormatTree(self));

        Ok(self.clone())
    }
}

impl Render<Geometries2D> for Models {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometries2D> {
        let mut geometries = Vec::new();
        for model in self.iter() {
            if let Some(geo) = model.render(context)? {
                geometries.push(geo);
            }
        }

        Ok(geometries.into_iter().collect())
    }
}

impl Render<Geometry2DOutput> for Models {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        Ok(match self.len() {
            0 => None,
            1 => self.first().expect("One item").render(context)?,
            _ => Some(Rc::new(Geometry2D::Collection(self.render(context)?))),
        })
    }
}

impl Render<Geometries3D> for Models {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometries3D> {
        let mut geometries = Vec::new();
        for model in self.iter() {
            if let Some(geo) = model.render(context)? {
                geometries.push(geo);
            }
        }

        Ok(geometries.into_iter().collect())
    }
}

impl Render<Geometry3DOutput> for Models {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        Ok(match self.len() {
            0 => None,
            1 => self.first().expect("One item").render(context)?,
            _ => Some(Rc::new(Geometry3D::Collection(self.render(context)?))),
        })
    }
}

impl Render<Geometry2DOutput> for BuiltinWorkpiece {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        Ok(match self.call()? {
            BuiltinWorkpieceOutput::Primitive2D(renderable) => {
                Some(renderable.render_to_geometry(&context.current_resolution()))
            }
            BuiltinWorkpieceOutput::Transform(transform) => {
                let model = context.model();
                let model_ = model.borrow();
                let geometry: Geometry2DOutput = model_.children.render(context)?;

                geometry.map(|geometry| {
                    Rc::new(
                        geometry.transformed_2d(&context.current_resolution(), &transform.mat2d()),
                    )
                })
            }
            BuiltinWorkpieceOutput::Operation(operation) => operation.process_2d(context)?,
            _ => None,
        })
    }
}

impl Render<Geometry3DOutput> for BuiltinWorkpiece {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        Ok(match self.call()? {
            BuiltinWorkpieceOutput::Primitive3D(geo3d) => Some(Rc::new(geo3d)),
            BuiltinWorkpieceOutput::Transform(transform) => {
                let model = context.model();
                let model_ = model.borrow();
                let geometry: Geometry3DOutput = model_.children.render(context)?;

                geometry.map(|geometry| {
                    Rc::new(
                        geometry.transformed_3d(&context.current_resolution(), &transform.mat3d()),
                    )
                })
            }
            BuiltinWorkpieceOutput::Operation(operation) => operation.process_3d(context)?,
            _ => None,
        })
    }
}
