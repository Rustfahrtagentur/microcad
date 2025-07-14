# `std::math`

## Mathematical functions

### Absolute Value (`abs(x)`)

Calculate absolute value:

[![test](.test/math_abs.png)](.test/math_abs.log)

```µcad,math_abs
std::debug::assert(std::math::abs(-1) == 1);
```

### Trigonometric functions: (`sin(x)`, `cos(x)`, `tan(x)`)

[![test](.test/math_trigonometric.png)](.test/math_trigonometric.log)

```µcad,math_trigonometric#todo
use std::debug::*;
use std::math::*;

assert_eq([cos(PI), -1.]);
assert_eq([tan(0), 0.]);
assert_eq([cot(PI/2), 0.]);

x = 0.5;
assert_eq([sin(x)^2. + cos(x)^2., 1.]);
```

### Rotation matrices

There are three functions to generate a rotation matrix as `Matrix3`.
The `Matrix3` is a built-in type.

#### `rotate_around_axis(angle: Angle, x: Scalar, y: Scalar, z: Scalar)`

Returns a rotation matrix for a rotation around an axis.

#### `rotate_xyz(x: Angle, y: Angle, z: Angle)`

Returns a rotation matrix for an Euler rotation around an X, Y, Z angles (in that order).
These angles are also called `roll`, `pitch` and `yaw`.

#### `rotate_zyx(x: Angle, y: Angle, z: Angle)`

Returns a rotation matrix for an Euler rotation around an Z, Y, X angles (in that order.)
These angles are also called `yaw`, `pitch` and `roll`.
