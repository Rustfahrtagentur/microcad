// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use geo::coord;
use microcad_core::*;
use microcad_lang::{eval::*, model::*, parameter, rc::*, value::*};

pub struct Rect;

impl BuiltinWorkbenchDefinition for Rect {
    fn id() -> &'static str {
        "Rect"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        let width: Scalar = args.get("width")?;
        let height: Scalar = args.get("height")?;
        let x = args.get("x")?;
        let y = args.get("y")?;

        Ok(
            ModelBuilder::new_2d_primitive(Rc::new(Geometry2D::Rect(geo2d::Rect::new(
                coord! {x: x, y: y},
                coord! {x: x + width, y: y + height},
            ))))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(width: Scalar),
            parameter!(height: Scalar),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
