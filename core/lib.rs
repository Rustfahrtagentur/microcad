// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core

mod boolean_op;

pub mod color;
pub mod core_error;
pub mod geo2d;
#[cfg(feature = "geo3d")]
pub mod geo3d;
pub mod render_resolution;
pub mod theme;

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

pub use boolean_op::BooleanOp;
pub use color::*;
pub use core_error::*;
pub use geo2d::*;
pub use geo3d::*;
pub use render_resolution::*;

include!(concat!(env!("OUT_DIR"), "/microcad_core_sin_cos.rs"));
