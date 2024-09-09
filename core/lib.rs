#![warn(missing_docs)]

pub mod algorithm;
pub mod error;
pub mod export;
pub mod geo2d;
#[cfg(feature = "geo3d")]
pub mod geo3d;
pub mod render;
pub mod transform;

pub type Integer = i64;
pub type Scalar = f64;
pub type Vec2 = cgmath::Vector2<Scalar>;
pub type Vec3 = cgmath::Vector3<Scalar>;
pub type Vec4 = cgmath::Vector4<Scalar>;
pub type Mat2 = cgmath::Matrix2<Scalar>;
pub type Mat3 = cgmath::Matrix3<Scalar>;
pub type Mat4 = cgmath::Matrix4<Scalar>;
pub type Angle = cgmath::Rad<Scalar>;

pub type Id = compact_str::CompactString;

pub use algorithm::Algorithm;
pub use error::Error;
pub use export::{ExportSettings, Exporter};
pub use transform::Transform;

pub type Result<T> = std::result::Result<T, Error>;
