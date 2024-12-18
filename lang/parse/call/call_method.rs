// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call method

use microcad_core::geo3d::Renderer;

use crate::{eval::*, objecttree::*, parse::*, src_ref::*};

/// Trait to call method of something.
/// This for example used to call `vertices()` on a `Node`.
/// In µcad, this is used to return vertices of a geometry, like a `rect` in this example:
/// ```uCAD
/// corners = std::geo2d::rect(size = 4mm).vertices();
/// ```
///
/// TODO: Should this really be in mod `parse`?
pub trait CallMethod {
    /// Call a method of a value
    ///
    /// - `name`: The name of the method, e.g. `vertices`.
    /// - `args`: Argument value for this method
    /// - `src_ref`: A source reference from which the method originates
    fn call_method(
        &self,
        name: &Identifier,
        args: &CallArgumentList,
        src_ref: SrcRef,
    ) -> EvalResult<Value>;
}

impl CallMethod for ObjectNode {
    fn call_method(
        &self,
        name: &Identifier,
        _args: &CallArgumentList,
        _src_ref: SrcRef,
    ) -> EvalResult<Value> {
        match name.into() {
            "volume" => {
                // Bake the object tree into a geometry tree
                let mut renderer = microcad_core::geo3d::MeshRenderer::new(0.1);
                let geometry_node = bake3d(&mut renderer, self.clone()).expect("Failed to bake");

                // Render the geometry tree into a triangle mesh
                renderer
                    .render_node(geometry_node)
                    .expect("Failed to render");

                // Return the volume of the triangle mesh
                Ok(Value::Scalar(Refer::none(renderer.triangle_mesh.volume())))
            }
            method_name => Err(EvalError::UnknownMethod(method_name.into())),
        }
    }
}

impl CallMethod for List {
    fn call_method(
        &self,
        name: &Identifier,
        _: &CallArgumentList,
        src_ref: SrcRef,
    ) -> EvalResult<Value> {
        match name.into() {
            "count" => Ok(Value::Integer(Refer::new(
                self.len() as i64,
                self.src_ref(),
            ))),
            "equal" => {
                let is_equal = match self.first() {
                    Some(first) => self[1..].iter().all(|x| x == first),
                    None => true,
                };
                Ok(Value::Bool(Refer::new(is_equal, src_ref)))
            }
            "ascending" => {
                let is_ascending = self.as_slice().windows(2).all(|w| w[0] <= w[1]);
                Ok(Value::Bool(Refer::new(is_ascending, src_ref)))
            }
            "descending" => {
                let is_descending = self.as_slice().windows(2).all(|w| w[0] >= w[1]);
                Ok(Value::Bool(Refer::new(is_descending, src_ref)))
            }
            method => Err(EvalError::UnknownMethod(method.into())),
        }
    }
}

#[test]
fn call_list_method() {
    let list = List::new(
        ValueList::new(
            vec![
                Value::Scalar(Refer::none(3.0)),
                Value::Scalar(Refer::none(3.0)),
                Value::Scalar(Refer::none(3.0)),
            ],
            SrcRef(None),
        ),
        crate::r#type::Type::Scalar,
        SrcRef(None),
    );

    if let Value::Bool(result) = list
        .call_method(&"equal".into(), &CallArgumentList::default(), SrcRef(None))
        .expect("test error")
    {
        assert!(result.value);
    } else {
        panic!("Test failed");
    }
}
