// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::{Id, eval::*, objects::ObjectNode, syntax::*};

/// Trait for calling methods of values
pub trait CallMethod {
    /// Evaluate method call into a value (if possible)
    ///
    /// - `name`: Name of the method
    /// - `args`: Arguments for the method
    /// - `context`: Evaluation context (TODO: It should not be optional)
    fn call_method(
        &self,
        name: &Id,
        args: &CallArgumentList,
        context: Option<&mut EvalContext>,
    ) -> EvalResult<Value>;
}

impl CallMethod for ObjectNode {
    fn call_method(
        &self,
        name: &Id,
        _: &CallArgumentList,
        _: Option<&mut EvalContext>,
    ) -> EvalResult<Value> {
        use crate::objects::bake3d;

        match name.as_str() {
            // Calculate volume from a 3D geometry
            "volume" => {
                // Bake the object tree into a geometry tree
                let precision = 0.1; // TODO get precision from context
                let mut renderer = microcad_core::geo3d::MeshRenderer::new(precision);
                
                if let Some(node3d) = bake3d(&mut renderer, self.clone()).expect("") {
                    use microcad_core::geo3d::Renderer;

                    // Render the geometry tree into a triangle mesh
                    renderer
                        .render_node(node3d)
                        .expect("Failed to render");
    
                    // Return the volume of the triangle mesh
                    Ok(Value::Scalar(Refer::none(renderer.triangle_mesh.volume())))
                } else {

                    Ok(Value::None)
                }

            }
            // Calculate area from a geometry
            // TODO Calculate surface area from a 3D geometry
            "area" => {
                todo!()
            }
            method_name => Err(EvalError::UnknownMethod(method_name.into())),
        }
    }
}

impl CallMethod for List {
    fn call_method(&self, name: &Id, _: &CallArgumentList, _: Option<&mut EvalContext>) -> EvalResult<Value> {
        match name.as_str() {
            "count" => Ok(Value::Integer(Refer::new(
                self.len() as i64,
                self.src_ref(),
            ))),
            "all_equal" => {
                let is_equal = match self.first() {
                    Some(first) => self[1..].iter().all(|x| x == first),
                    None => true,
                };
                Ok(Value::Bool(Refer::none(is_equal)))
            }
            "is_ascending" => {
                let is_ascending = self.as_slice().windows(2).all(|w| w[0] <= w[1]);
                Ok(Value::Bool(Refer::none(is_ascending)))
            }
            "is_descending" => {
                let is_descending = self.as_slice().windows(2).all(|w| w[0] >= w[1]);
                Ok(Value::Bool(Refer::none(is_descending)))
            }
            method => Err(EvalError::UnknownMethod(method.into())),
        }
    }
}

impl CallMethod for Value {
    fn call_method(
            &self,
            name: &Id,
            args: &CallArgumentList,
            context: Option<&mut EvalContext>,
        ) -> EvalResult<Value> {
        match &self {
            Value::None => todo!(),
            Value::Integer(refer) => todo!(),
            Value::Scalar(refer) => todo!(),
            Value::Length(refer) => todo!(),
            Value::Area(refer) => todo!(),
            Value::Volume(refer) => todo!(),
            Value::Vec2(refer) => todo!(),
            Value::Vec3(refer) => todo!(),
            Value::Vec4(refer) => todo!(),
            Value::Angle(refer) => todo!(),
            Value::Weight(refer) => todo!(),
            Value::Bool(refer) => todo!(),
            Value::String(refer) => todo!(),
            Value::Color(refer) => todo!(),
            Value::List(list) => todo!(),
            Value::Map(map) => todo!(),
            Value::NamedTuple(named_tuple) => todo!(),
            Value::UnnamedTuple(unnamed_tuple) => todo!(),
            Value::Node(node) => node.call_method(name, args, context),
            Value::NodeMultiplicity(nodes) => object_with_children(nodes).call_method(name, args, context),
            _ => todo!()
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
        crate::ty::Type::Scalar,
        SrcRef(None),
    );

    if let Value::Bool(result) = list
        .call_method(&"all_equal".into(), &CallArgumentList::default(), None)
        .expect("test error")
    {
        assert!(result.value);
    } else {
        panic!("Test failed");
    }
}
