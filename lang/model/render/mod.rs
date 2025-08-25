// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

mod cache;

use std::rc::Rc;

pub use cache::*;

use microcad_core::*;
use thiserror::Error;

use crate::{model::*, value::ValueError};

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
        self.borrow().output.output_type()
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
        self.borrow().children.iter().try_fold(
            Default::default(),
            |mut geometries: Geometries2D, model| {
                let model_ = model.borrow();
                let mat = model_.output.local_matrix_2d();
                geometries.push(
                    model
                        .process_2d(cache, model)?
                        .transformed_2d(&model_.output.resolution, &mat),
                );
                Ok(geometries)
            },
        )
    }

    /// Render geometries in 3D.
    pub fn render_geometry_3d(&self, _cache: &mut RenderCache) -> RenderResult<Geometries3D> {
        todo!()
    }

    /// Render the model.
    ///
    /// Rendering the model means that all geometry is calculated and stored
    /// in the in the render cache.
    pub fn render(&self, resolution: RenderResolution, _cache: &mut RenderCache) {
        /// Set the resolution for this model.
        pub fn set_resolution(model: &Model, resolution: RenderResolution) {
            let new_resolution = {
                let mut model_ = model.borrow_mut();
                let new_resolution = resolution * model_.output.world_matrix;
                model_.output.resolution = new_resolution.clone();
                new_resolution
            };

            model.borrow().children.iter().for_each(|model| {
                set_resolution(model, new_resolution.clone());
            });
        }

        //set_matrix(self, Mat4::identity());
        set_resolution(self, resolution);

        todo!("Implement render algorithm")
    }
}

impl Operation for Model {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry2D>> {
        let mut geometries = Geometries2D::default();

        let model_ = &model.borrow();
        match &*model_.element {
            Element::Group | Element::Workpiece(_) => {
                model_.children().try_for_each(|n| -> RenderResult<_> {
                    geometries.push(n.process_2d(cache, n)?.as_ref().clone());
                    Ok(())
                })?;
            }
            Element::BuiltinWorkpiece(builtin_workpiece) => {
                geometries.push(builtin_workpiece.call_2d(cache, model)?.as_ref().clone());
                // TODO: Pass children geometry, too?
                //model_
                //    .children()
                //    .for_each(|n| geometries.push(n.process_2d(cache, n).as_ref().clone()));
            }
            _ => {}
        }

        Ok(Rc::new(Geometry2D::Collection(geometries.transformed_2d(
            &model_.output.resolution,
            &model_.output.local_matrix_2d(),
        ))))
    }

    fn process_3d(&self, _cache: &mut RenderCache, _model: &Model) -> RenderResult<Rc<Geometry3D>> {
        todo!();
    }
}

impl FetchBounds2D for Model {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        let mut bounds = Bounds2D::default();

        self.descendants().for_each(|model| {
            let output = &model.borrow().output;
            if let GeometryOutput::Geometry2D(Some(geometry)) = &output.geometry {
                let mat = output.world_matrix_2d();
                let resolution = &output.resolution;
                bounds = bounds
                    .clone()
                    .extend(geometry.fetch_bounds_2d().transformed_2d(resolution, &mat));
            }
        });

        bounds
    }
}

impl FetchBounds3D for Model {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        self.descendants()
            .fold(Bounds3D::default(), |mut bounds, model| {
                let output = &model.borrow().output;
                if let GeometryOutput::Geometry3D(Some(geometries)) = &output.geometry {
                    let mat = output.world_matrix_3d();
                    let resolution = &output.resolution;
                    bounds = bounds.clone().extend(
                        geometries
                            .fetch_bounds_3d()
                            .transformed_3d(resolution, &mat),
                    );
                }
                bounds
            })
    }
}
