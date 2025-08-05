// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Write primitives to STL ([`WriteSvg`] trait implementations).

use microcad_core::{Geometry3D, Manifold, Transformed3D, TriangleMesh};
use microcad_lang::model::{Element, Model, OutputType};

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
        assert_eq!(self.final_output_type(), OutputType::Geometry3D);

        let self_ = self.borrow();
        let world_matrix = self_.output.world_matrix;
        let render_resolution = self_.output.resolution.clone();

        // Render all output geometries.
        self.fetch_output_geometries_3d()
            .iter()
            .try_for_each(|geometry| {
                geometry
                    .transformed_3d(&render_resolution, &world_matrix)
                    .write_stl(writer)
            })?;

        if !matches!(self_.element, Element::Operation(_)) {
            self_
                .children()
                .try_for_each(|child| child.write_stl(writer))?;
        }

        Ok(())
    }
}
