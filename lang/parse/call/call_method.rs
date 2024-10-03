// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call method

use crate::{eval::*, parse::*, src_ref::*};

/// Trait to call method of something.
/// This for example used to call `vertices()` on a `Node`.
/// In µCAD, this is used to return vertices of a geometry, like a `rect` in this example:
/// ```µCAD
/// corners = std::geo2d::rect(size = 4mm).vertices();
/// ```
pub trait CallMethod {
    /// Call a method of a value
    ///
    /// - `name`: The name of the method, e.g. `vertices`.
    /// - `args`: Argument value for this method
    /// - `src_ref`: A source reference from which the method originates
    fn call_method(
        &self,
        name: &Identifier,
        args: &CallArgumentValueList,
        src_ref: SrcRef,
    ) -> Result<Value>;
}

impl CallMethod for microcad_render::Node {
    fn call_method(
        &self,
        name: &Identifier,
        args: &CallArgumentValueList,
        src_ref: SrcRef,
    ) -> Result<Value> {
        use microcad_render::NodeInner;

        match name.into() {
            // Return the vertices of a node
            "vertices" => {
                let parameter_values = ParameterValueList::default();
                let _arg_map = args.get_matching_arguments(&parameter_values)?;
                let inner = self.borrow();
                match &*inner {
                    NodeInner::Geometry2D(geo) => Ok(geo.vertices().into_value(src_ref)),
                    _ => Err(EvalError::UnknownMethod("vertices".into())),
                }
            }
            method_name => Err(EvalError::UnknownMethod(method_name.into())),
        }
    }
}

impl CallMethod for List {
    fn call_method(
        &self,
        name: &Identifier,
        _: &CallArgumentValueList,
        src_ref: SrcRef,
    ) -> Result<Value> {
        match name.into() {
            "equal" => {
                let result = match self.first() {
                    Some(first) => self.iter().all(|x| x == first),
                    None => true,
                };
                Ok(Value::Bool(Refer::new(result, src_ref)))
            }
            method => Err(EvalError::UnknownMethod(method.into())),
        }
    }
}

#[test]
fn call_method() {
    use microcad_core::geo2d::Rect;
    use microcad_render::{Node, NodeInner};
    let node = Node::new(NodeInner::Geometry2D(
        microcad_core::geo2d::Geometry::Rect(Rect::new(
            microcad_core::geo2d::coord! { x: 10., y: 20. },
            microcad_core::geo2d::coord! { x: 30., y: 10. },
        ))
        .into(),
    ));

    let value = node
        .call_method(
            &"vertices".into(),
            &CallArgumentValueList::default(),
            SrcRef(None),
        )
        .unwrap();
    if let Value::List(value_list) = value {
        // We expect a [(x: length, y: length)]
        assert_eq!(value_list.ty(), crate::r#type::Type::Vec2);

        // A rect as 4 corners
        assert_eq!(value_list.len(), 4);
    } else {
        panic!("Expected a list of values");
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
        .call_method(
            &"equal".into(),
            &CallArgumentValueList::default(),
            SrcRef(None),
        )
        .unwrap()
    {
        assert!(result.value);
    } else {
        panic!("Test failed");
    }
}
