// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use derive_more::DerefMut;

pub struct RenderCache {}

impl RenderCache {
    fn render(&mut self, model: &Model, parameters: &RenderParameters) -> Vec<Hash> {}
}

pub trait Render {}

impl Render for Models {}

struct RenderOutput {
    output_type: OutputType,

    /// Local transformation matrix.
    pub local_matrix: Mat4,

    /// World transformation matrix.
    pub world_matrix: Mat4,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: RenderResolution,

    /// Hash to the output geometry.
    pub output_hash: Hash,
}

pub struct ModelNodeInner {
    // Generated via eval()
    id: Option<Identifier>,
    pub parent: Option<Model>,
    origin: Refer<Origin>,
    attributes: Attributes,
    children: Models,

    // Calculated before rendering.
    // Determine output type
    // Determine matrix
    // Determine render resolutions
    output: Option<RenderOutput>,
}

impl Builtin {
    fn call(&self, cache: &mut RenderCache, args: Tuple) -> RenderResult<BuiltinResult> {}
}

/// BuiltinWorkbench symbol
pub enum BuiltinResult {
    Value(Value),

    Transform(AffineTransform),

    /// A 2D geometry.
    Primitive2D(std::rc::Rc<Geometry2D>),

    /// A 3D geometry.
    Primitive3D(std::rc::Rc<Geometry3D>),

    /// An operation that generates geometries from its children.
    #[serde(skip)]
    Operation(std::rc::Rc<dyn Operation>),
}

impl BuiltinType {
    fn call(&self, cache: &mut RenderCache) -> Hash {}
}
