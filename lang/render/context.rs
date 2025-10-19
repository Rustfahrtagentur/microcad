// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render context

use microcad_core::RenderResolution;

use crate::{model::Model, rc::RcMut, render::*};

/// The render context.
///
/// Keeps a stack of model nodes and the render cache.
#[derive(Default)]
pub struct RenderContext {
    /// Model stack.
    pub model_stack: Vec<Model>,

    /// Optional render cache.
    pub cache: Option<RcMut<RenderCache>>,
}

impl RenderContext {
    /// Create default context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize context with current model and prerender model.
    pub fn init(
        model: &Model,
        resolution: RenderResolution,
        cache: Option<RcMut<RenderCache>>,
    ) -> RenderResult<Self> {
        model.prerender(resolution)?;
        Ok(Self {
            model_stack: vec![model.clone()],
            cache,
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
    pub fn update_2d<T: Into<WithBounds2D<Geometry2D>>>(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<T>,
    ) -> RenderResult<Geometry2DOutput> {
        let model = self.model();
        let hash = model.computed_hash();

        match self.cache.clone() {
            Some(cache) => {
                {
                    let mut cache = cache.borrow_mut();
                    if let Some(GeometryOutput::Geometry2D(geo)) = cache.get(&hash) {
                        return Ok(geo.clone());
                    }
                }
                {
                    let geo: Geometry2DOutput = Rc::new(f(self, model)?.into());
                    let mut cache = cache.borrow_mut();
                    cache.insert(hash, geo.clone());
                    Ok(geo)
                }
            }
            None => Ok(Rc::new(f(self, model)?.into())),
        }
    }

    /// Update a 3D geometry if it is not in cache.
    pub fn update_3d<T: Into<WithBounds3D<Geometry3D>>>(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<T>,
    ) -> RenderResult<Geometry3DOutput> {
        let model = self.model();
        let hash = model.computed_hash();
        match self.cache.clone() {
            Some(cache) => {
                {
                    let mut cache = cache.borrow_mut();
                    if let Some(GeometryOutput::Geometry3D(geo)) = cache.get(&hash) {
                        return Ok(geo.clone());
                    }
                }
                {
                    let geo: Geometry3DOutput = Rc::new(f(self, model)?.into());
                    let mut cache = cache.borrow_mut();
                    cache.insert(hash, geo.clone());
                    Ok(geo)
                }
            }
            None => Ok(Rc::new(f(self, model)?.into())),
        }
    }

    /// Return current render resolution.
    pub fn current_resolution(&self) -> RenderResolution {
        self.model().borrow().resolution()
    }
}
