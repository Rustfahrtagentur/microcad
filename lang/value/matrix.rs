// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Matrix value type

use crate::ty::*;

/// Matrix type
#[derive(Debug, Clone, PartialEq)]
pub enum Matrix {
    /// 2x2 matrix.
    Matrix2(microcad_core::Mat2),
    /// 3x3 matrix.
    Matrix3(microcad_core::Mat3),
    /// 4x4 matrix.
    Matrix4(microcad_core::Mat4),
}

impl Ty for Matrix {
    fn ty(&self) -> Type {
        match self {
            Matrix::Matrix2(_) => Type::Matrix(MatrixType::new(2, 2)),
            Matrix::Matrix3(_) => Type::Matrix(MatrixType::new(3, 3)),
            Matrix::Matrix4(_) => Type::Matrix(MatrixType::new(4, 4)),
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Matrix::Matrix2(matrix2) => write!(f, "{matrix2:?}"),
            Matrix::Matrix3(matrix3) => write!(f, "{matrix3:?}"),
            Matrix::Matrix4(matrix4) => write!(f, "{matrix4:?}"),
        }
    }
}
