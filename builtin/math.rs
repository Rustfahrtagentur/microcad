// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::{Mat3, Vec3};
use microcad_lang::{diag::*, eval::*, parameter, resolve::*, syntax::*, ty::*, value::*};

/// Absolute value abs(x)
fn abs() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("abs"), None, &|_params, args, ctx| {
        let (_, arg) = args.get_single()?;
        Ok(match &arg.value {
            Value::Integer(i) => Value::Integer(i.abs()),
            Value::Quantity(q) => {
                Value::Quantity(Quantity::new(q.value.abs(), q.quantity_type.clone()))
            }
            value => {
                ctx.error(
                    arg,
                    EvalError::BuiltinError(format!("Cannot calculate abs({value})")),
                )?;
                Value::None
            }
        })
    })
}

/// Implementation for a builtin trigonometric function.
fn trigonometric(
    name: &str,
    args: &ArgumentValueList,
    ctx: &mut Context,
    f: impl FnOnce(f64) -> f64,
) -> EvalResult<Value> {
    let (_, arg) = args.get_single()?;
    Ok(match &arg.value {
        Value::Integer(i) => Value::Quantity(Quantity::new(f(*i as f64), QuantityType::Scalar)),
        Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Angle,
        })
        | Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Scalar,
        }) => Value::Quantity(Quantity::new(f(*value), QuantityType::Scalar)),
        value => {
            ctx.error(
                arg,
                EvalError::BuiltinError(format!("Cannot calculate {name}({value})")),
            )?;
            Value::None
        }
    })
}

/// Calculate cos(x).
fn cos() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("cos"), None, &|_params, args, ctx| {
        trigonometric("cos", args, ctx, |v| v.cos())
    })
}

/// Calculate sin(x).
fn sin() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("sin"), None, &|_params, args, ctx| {
        trigonometric("sin", args, ctx, |v| v.sin())
    })
}

/// Calculate tan(x).
fn tan() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("tan"), None, &|_params, args, ctx| {
        trigonometric("tan", args, ctx, |v| v.tan())
    })
}

/// Helper function to get an angle from a field in an argument list.
fn get_angle(args: &Tuple, axis: &str) -> cgmath::Deg<f64> {
    match args.get_value(axis).expect("angle missing") {
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
                let axis = Vec3::new(args.get("x")?, args.get("y")?, args.get("z")?);

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
        .symbol(cos())
        .symbol(sin())
        .symbol(tan())
        .symbol(rotate_around_axis())
        .symbol(rotate_xyz())
        .symbol(rotate_zyx())
        .build()
}
