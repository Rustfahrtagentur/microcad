// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use microcad_core::{Geometries2D, Geometries3D};

use crate::model::GeometryOutput;

pub struct RenderCache(std::collections::HashMap<crate::Hash, std::rc::Rc<GeometryOutput>>);
