// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

pub mod element;
pub mod metadata;
pub mod modelnode;
pub mod modelnodes;
pub mod object;
pub mod transformation;

pub use element::*;
pub use metadata::*;
pub use modelnode::*;
pub use modelnodes::*;
pub use object::*;
pub use transformation::*;

/*
enum GeometryType {
    Geometry2D,
    Geometry3D,
}

enum ProcessingState {
    /// The [`ModelNode`] has not been processed, it is *raw*.
    ///
    /// E.g., this is the state when the [`ModelNode`] is the result of evaluating a source file.
    Raw,

    ///
    OutputType,


    /// The Render
    ///
    /// * The transformation matrices have been calculated.
    AffineTransformInfo,

    /// Generate geometry for primitives.
    Primitives,

    /// Generate geometry for transformations.
    Transformations,
}


struct AffineTransformInfo {
    transform: AffineTransform,
    local_matrix: Mat4,
    world_matrix: Mat4,
    precision: Scalar,
}

impl AffineTransformInfo {
    fn new(node: ModelNode) -> Self {
        match node.parent() {
            None => AffineTransform {
                transform: AffineTransform::None,

            }
        }
    }
}


*/
