// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render context

use microcad_core::RenderResolution;

use crate::{model::Model, render::*};

/// The render context.
///
/// Keeps a stack of model nodes and the render cache.
#[derive(Default)]
pub struct RenderContext {
    /// Model stack.
    pub model_stack: Vec<Model>,

    /// Render cache.
    pub cache: RenderCache,
}

impl RenderContext {
    /// Create default context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize context with current model and prerender model.
    pub fn init(model: &Model, resolution: RenderResolution) -> RenderResult<Self> {
        model.prerender(resolution)?;
        Ok(Self {
            model_stack: vec![model.clone()],
            ..Default::default()
        })
    }

    /// The current model (panics if it is none).
    pub fn model(&self) -> Model {
        self.model_stack.last().expect("A model").clone()
    }

    /// Run the closure `f` within the given `model`.
    pub fn with_model<T>(&mut self, model: Model, f: impl FnOnce(&mut RenderContext) -> T) -> T {
        self.model_stack.push(model);
        let result = f(self);
        self.model_stack.pop();
        result
    }

    /// Update a 2D geometry if it is not in cache.
    pub fn update_2d(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<Geometry2DOutput>,
    ) -> RenderResult<Geometry2DOutput> {
        let model = self.model();
        let hash = model.computed_hash();
        match self.cache.get_2d(&hash) {
            Some(geo) => Ok(geo.clone()),
            None => {
                let geo = f(self, model)?;
                self.cache.insert_2d(hash, geo.clone());
                Ok(geo)
            }
        }
    }

    /// Update a 3D geometry if it is not in cache.
    pub fn update_3d(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<Geometry3DOutput>,
    ) -> RenderResult<Geometry3DOutput> {
        let model = self.model();
        let hash = model.computed_hash();
        match self.cache.get_3d(&hash) {
            Some(geo) => Ok(geo.clone()),
            None => {
                let geo = f(self, model)?;
                self.cache.insert_3d(hash, geo.clone());
                Ok(geo)
            }
        }
    }

    /// Return current render resolution.
    pub fn current_resolution(&self) -> RenderResolution {
        self.model().borrow().resolution()
    }
}
