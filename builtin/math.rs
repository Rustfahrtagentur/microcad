// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::{Mat3, Vec3};
use microcad_lang::{diag::*, eval::*, parameter, resolve::*, syntax::*, ty::*, value::*};

/// Absolute value abs(x)
fn abs() -> Symbol {
    let id = Identifier::no_ref("abs");
    Symbol::new_builtin(id, None, &|_params, args, ctx| {
        let (id, arg) = args.get_single()?;
        Ok(match &arg.value {
            Value::Integer(i) => Value::Integer(i.abs()),
            Value::Quantity(q) => {
                Value::Quantity(Quantity::new(q.value.abs(), q.quantity_type.clone()))
            }
            value => {
                ctx.error(
                    arg,
                    EvalError::TypeMismatch {
                        id: id.clone(),
                        expected: Type::Integer,
                        found: value.ty(),
                    },
                )?;
                Value::None
            }
        })
    })
}

/// Helper function to get an angle from a field in an argument list.
fn get_angle(args: &Tuple, axis: &str) -> cgmath::Deg<f64> {
    match args.get_value(axis) {
        Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Angle,
        }) => cgmath::Deg::<f64>(*value),
        _ => unreachable!(),
    }
}

/// Helper function to return rotation X,Y,Z rotation matrices from an [`Tuple`].
fn rotation_matrices_xyz(args: &Tuple) -> (Mat3, Mat3, Mat3) {
    (
        Mat3::from_angle_x(get_angle(args, "x")),
        Mat3::from_angle_y(get_angle(args, "y")),
        Mat3::from_angle_z(get_angle(args, "z")),
    )
}

/// Rotate a vector around an axis.
fn rotate_around_axis() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("rotate_around_axis"),
        Some(
            [
                parameter!(angle: Angle),
                parameter!(x: Scalar),
                parameter!(y: Scalar),
                parameter!(z: Scalar),
            ]
            .into_iter()
            .collect(),
        ),
        &|_params, args, ctx| match ArgumentMatch::find_match(
            args,
            _params.expect("ParameterValueList"),
        ) {
            Ok(ref args) => {
                let angle = get_angle(args, "angle");
                let axis = Vec3::new(args.get("x"), args.get("y"), args.get("z"));

                let matrix = Mat3::from_axis_angle(axis, angle);
                Ok(Value::Matrix(Box::new(Matrix::Matrix3(matrix))))
            }
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
    )
}

/// Rotate around X, Y, Z (in that order).
fn rotate_xyz() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("rotate_xyz"),
        Some(
            [
                parameter!(x: Angle),
                parameter!(y: Angle),
                parameter!(z: Angle),
            ]
            .into_iter()
            .collect(),
        ),
        &|_params, args, ctx| match ArgumentMatch::find_match(
            args,
            _params.expect("ParameterValueList"),
        ) {
            Ok(args) => {
                let (x_matrix, y_matrix, z_matrix) = rotation_matrices_xyz(&args);
                Ok(Value::Matrix(Box::new(Matrix::Matrix3(
                    x_matrix * y_matrix * z_matrix,
                ))))
            }
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
    )
}

/// Rotate around Z, Y, X (in that order).
fn rotate_zyx() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("rotate_zyx"),
        Some(
            [
                parameter!(x: Angle),
                parameter!(y: Angle),
                parameter!(z: Angle),
            ]
            .into_iter()
            .collect(),
        ),
        &|_params, args, ctx| match ArgumentMatch::find_match(
            args,
            _params.expect("ParameterValueList"),
        ) {
            Ok(args) => {
                let (x_matrix, y_matrix, z_matrix) = rotation_matrices_xyz(&args);
                Ok(Value::Matrix(Box::new(Matrix::Matrix3(
                    z_matrix * y_matrix * x_matrix,
                ))))
            }
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
    )
}

pub fn math() -> Symbol {
    crate::ModuleBuilder::new("math".try_into().expect("unexpected name error"))
        .symbol(Symbol::new_constant(
            Identifier::no_ref("PI"),
            Value::Quantity(Quantity::new(std::f64::consts::PI, QuantityType::Scalar)),
        ))
        .symbol(Symbol::new_constant(
            Identifier::no_ref("X"),
            Value::Tuple(Box::new(Vec3::unit_x().into())),
        ))
        .symbol(Symbol::new_constant(
            Identifier::no_ref("Y"),
            Value::Tuple(Box::new(Vec3::unit_y().into())),
        ))
        .symbol(Symbol::new_constant(
            Identifier::no_ref("Z"),
            Value::Tuple(Box::new(Vec3::unit_z().into())),
        ))
        .symbol(abs())
        .symbol(rotate_around_axis())
        .symbol(rotate_xyz())
        .symbol(rotate_zyx())
        .build()
}
