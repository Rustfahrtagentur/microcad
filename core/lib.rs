// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD core

#![warn(missing_docs)]

pub mod algorithm;
pub mod error;
pub mod export;
pub mod geo2d;
#[cfg(feature = "geo3d")]
pub mod geo3d;
pub mod render;
pub mod transform;

/// Primitive integer type
pub type Integer = i64;
/// Primitive floating point type
pub type Scalar = f64;
/// 2D vector type
pub type Vec2 = cgmath::Vector2<Scalar>;
/// 3D vector type
pub type Vec3 = cgmath::Vector3<Scalar>;
/// 4D vector type
pub type Vec4 = cgmath::Vector4<Scalar>;
/// 2D matrix type
pub type Mat2 = cgmath::Matrix2<Scalar>;
/// 3D matrix type
pub type Mat3 = cgmath::Matrix3<Scalar>;
/// 4D matrix type
pub type Mat4 = cgmath::Matrix4<Scalar>;
/// Primitive angle type
pub type Angle = cgmath::Rad<Scalar>;
/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

pub use algorithm::Algorithm;
pub use error::CoreError;
pub use export::{ExportSettings, Exporter};
pub use transform::Transform;

/// Core result type
pub type Result<T> = std::result::Result<T, CoreError>;


/// Trait to calculate depth for a node
pub trait Depth {
    /// Calculate depth
    fn depth(&self) -> usize;
}
