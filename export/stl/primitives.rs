// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Write primitives to STL ([`WriteSvg`] trait implementations).

use microcad_core::{Geometry3D, Manifold, Transformed3D, TriangleMesh};
use microcad_lang::model::Model;

use crate::stl::{StlWriter, WriteStl};

impl WriteStl for TriangleMesh {
    fn write_stl(&self, writer: &mut StlWriter) -> std::io::Result<()> {
        self.triangles()
            .try_for_each(|tri| writer.write_triangle(&tri))
    }
}

impl WriteStl for Manifold {
    fn write_stl(&self, writer: &mut StlWriter) -> std::io::Result<()> {
        let triangle_mesh: TriangleMesh = self.to_mesh().into();
        triangle_mesh.write_stl(writer)
    }
}

impl WriteStl for Geometry3D {
    fn write_stl(&self, writer: &mut StlWriter) -> std::io::Result<()> {
        match self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.write_stl(writer),
            Geometry3D::Manifold(manifold) => manifold.write_stl(writer),
            _ => unreachable!("Can only write triangle geometries to STL"),
        }
    }
}

impl WriteStl for Model {
    fn write_stl(&self, writer: &mut StlWriter) -> std::io::Result<()> {
        let self_ = self.borrow();
        let output = self_.output();
        match output {
            microcad_lang::render::RenderOutput::Geometry3D {
                world_matrix,
                geometry,
                ..
            } => {
                let mat = world_matrix.expect("Some matrix");
                match geometry {
                    Some(geometry) => geometry
                        .transformed_3d(&self_.resolution(), &mat)
                        .write_stl(writer),
                    None => self_
                        .children()
                        .try_for_each(|model| model.write_stl(writer)),
                }
            }
            _ => Ok(()),
        }
    }
}
