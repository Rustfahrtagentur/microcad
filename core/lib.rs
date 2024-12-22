// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core

const DEFAULT_RENDERING_PRECISION: f64 = 0.1;

mod boolean_op;
pub mod core_error;
pub mod geo2d;
#[cfg(feature = "geo3d")]
pub mod geo3d;

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

use std::str::FromStr;

pub use boolean_op::BooleanOp;
pub use core_error::*;

/// Trait to calculate depth for a node
pub trait Depth {
    /// Calculate depth
    fn depth(&self) -> usize;
}

/// Render hash trait
pub trait RenderHash {
    /// Calculate a hash of self
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

/// Renderer trait
pub trait Renderer {
    /// The precision of the renderer in mm
    fn precision(&self) -> crate::Scalar;

    /// Change the render state
    fn change_render_state(&mut self, _: &str, _: &str) -> CoreResult<()> {
        Ok(())
    }
}

/// 2D Renderer type alias
pub type Renderer2D = dyn geo2d::Renderer;

/// 3D Renderer type alias
pub type Renderer3D = dyn geo3d::Renderer;

/// 2D Primitive type alias
pub type Primitive2D = dyn geo2d::Primitive;

/// 3D Primitive type alias
pub type Primitive3D = dyn geo3d::Primitive;

/// Export settings, essentially a TOML table
#[derive(Debug, Default, Clone)]
pub struct ExportSettings(toml::Table);

impl std::ops::Deref for ExportSettings {
    type Target = toml::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExportSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ExportSettings {
    /// Create export settings with an initial file name
    pub fn with_filename(filename: String) -> Self {
        let mut settings = ExportSettings::default();
        settings.insert("filename".to_string(), toml::Value::String(filename));
        settings
    }

    /// return file name
    pub fn file_path(&self) -> CoreResult<std::path::PathBuf> {
        match self.get("filename") {
            Some(filename) => Ok(std::path::PathBuf::from_str(
                filename.as_str().expect("Filename must be a string"),
            )?),
            None => Err(CoreError::NoFilenameSpecifiedForExport),
        }
    }
    /// Return render precision
    pub fn render_precision(&self) -> CoreResult<f64> {
        if let Some(precision) = self.0.get("render_precision") {
            if let Some(precision) = precision.as_float() {
                Ok(precision)
            } else {
                Err(CoreError::InvalidRenderPrecision(precision.to_string()))
            }
        } else {
            Ok(DEFAULT_RENDERING_PRECISION)
        }
    }

    /// Get exporter ID
    pub fn exporter_id(&self) -> CoreResult<Option<String>> {
        if let Some(exporter) = self.0.get("exporter") {
            Ok(Some(exporter.to_string()))
        } else {
            Ok(self
                .file_path()?
                .extension()
                .map(|p| p.to_string_lossy().to_string()))
        }
    }
}

#[test]
fn export_settings() {
    let export_settings = ExportSettings::with_filename("test.stl".into());

    assert_eq!(
        export_settings.file_path().expect("test error"),
        std::path::PathBuf::from_str("test.stl").unwrap()
    );
    assert_eq!(
        export_settings.exporter_id().expect("test error"),
        Some("stl".into())
    );
}
