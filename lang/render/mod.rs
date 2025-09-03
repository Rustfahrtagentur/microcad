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
pub enum RenderError {}

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
                model_.output = output;
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

                let resolution = resolution * output.world_matrix();
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

/// This implementation renders a [`Geometry2D`] out of a [`Model`].
///
/// Note: It does *not* attach the output geometry to the model's render output.
/// It is assumed the model has been pre-rendered.
/// The current model in the context has not changed.
/// If the geometry is already in the cache, the geometry in the cache will be returned.
impl Render<Geometry2DOutput> for Model {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.with_model(self.clone(), |context| {
            let model = context.model();
            let geometry = {
                let model_ = model.borrow();
                let output = model_.output();
                match output {
                    RenderOutput::Geometry2D { .. } => {
                        match model_.element() {
                            // A group geometry will render the child geometry
                            Element::Group => model_.children.render(context),
                            Element::Workpiece(workpiece) => workpiece.render(context),
                            Element::BuiltinWorkpiece(builtin_workpiece) => {
                                builtin_workpiece.render(context)
                            }
                            Element::ChildrenMarker => unreachable!(),
                        }
                    }
                    RenderOutput::Geometry3D { .. } => unreachable!(),
                }
            }?;

            self.borrow_mut()
                .output_mut()
                .set_geometry_2d(geometry.clone());
            Ok(geometry)
        })
    }
}

impl Render<Model> for Model {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Model> {
        let _: Geometry2DOutput = self.render(context)?;
        log::trace!("Finished render:\n{}", FormatTree(self));

        Ok(self.clone())
    }
}

impl Render<Geometries2D> for Models {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometries2D> {
        let mut geometries = Vec::new();
        for model in self.iter() {
            if let Some(geo) = model.render(context)? {
                geometries.push(geo.as_ref().clone());
            }
        }

        Ok(Geometries2D::new(geometries))
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

impl Render<Geometry2DOutput> for BuiltinWorkpiece {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        Ok(match self.call()? {
            BuiltinWorkpieceOutput::Geometry2D(geo2d) => Some(Rc::new(geo2d)),
            BuiltinWorkpieceOutput::Operation(operation) => operation.process_2d(context)?,
            _ => None,
        })
    }
}

impl Render<Geometry2DOutput> for Workpiece {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        let model = context.model();
        let model_ = model.borrow();
        model_.children.render(context)
    }
}
