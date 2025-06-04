# Primitive Types

## Integer

The type `integer` contains a natural number.

[![test](.test/types_primitive_integer.png)](.test/types_primitive_integer.log)

```µcad,types_primitive_integer
i = 3;
```

## Scalar

The type `Scalar` contains a floating number and must be written with at least one decimal place (or in percent).

[![test](.test/types_primitive_scalar.png)](.test/types_primitive_scalar.log)

```µcad,types_primitive_scalar
zero = 0;
pi =  3.1415;
percent = 55%;
```

## Length

Length are used in describing dimensions and must be given with a unit.

[![test](.test/types_primitive_length.png)](.test/types_primitive_length.log)

```µcad,types_primitive_length
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37007874015748in;

std::debug::assert( [millimeters, centimeters, meters, inches].all_equal() );
```

## Angle

Angles are used with rotations and in constrains when proving measures.

[![test](.test/types_primitive_angle.png)](.test/types_primitive_angle.log)

```µcad,types_primitive_angle
pi = std::math::pi;
radian = 1rad * pi;
degree = 180°;
degree_ = 180deg;
grad = 200grad;
turn = 0.5turn;

std::debug::assert( [degree, degree_, grad, turn, radian].all_equal() );
```

## Area

[![test](.test/types_primitive_area.png)](.test/types_primitive_area.log)

```µcad,types_primitive_area
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in²;

std::debug::assert(square_millimeter == 0.1m²);
std::debug::assert(square_centimeter == 0.1m²);
```

## Volume

[![test](.test/types_primitive_volume.png)](.test/types_primitive_volume.log)

```µcad,types_primitive_volume
cubic_millimeter = 1000000.0mm³;
cubic_centimeter = 100.0cl;
cubic_meter = 0.001m³;
cubic_inch = 61.0237in³;
liter = 1.0l;
centiliter = 100.0cl;
milliliter = 1000.0ml;

std::debug::assert(cubic_millimeter == 1.0l);
std::debug::assert(cubic_centimeter == 1.0l);
std::debug::assert(cubic_meter == 1.0l);
std::debug::assert(centiliter == 1.0l);
std::debug::assert(milliliter == 1.0l);
```

## String

Text can be used to logging or to render text.

[![test](.test/types_primitive_string.png)](.test/types_primitive_string.log)

```µcad,types_primitive_string#todo
text = "Hello µcad!";
std::debug::assert(text.count(11));

// logging
std::info(text);

// render text
std::geo2d::text(text);
```

## Weight

Weights can be calculated by applying volumes to materials.

[![test](.test/types_primitive_weight.png)](.test/types_primitive_weight.log)

```µcad,types_primitive_weight
gram = 1000.0g;
kilogram = 1.0kg;
pound = 2.204623lb;

std::debug::assert(gram == 1.0kg);
```

### Vec2

2D vectors are written as named tuples with builtin values `x` and `y`.

[![test](.test/types_primitive_vec2.png)](.test/types_primitive_vec2.log)

```µcad,types_primitive_vec2
vec2 = (x=1, y=2)cm;
```

## Vec3

3D vectors are written as named tuples with builtin values `x`, `y` and `z`.

[![test](.test/types_primitive_vec3.png)](.test/types_primitive_vec3.log)

```µcad,types_primitive_vec3
vec3 = (x=1, y=2, z=3)cm;
```

## Vec4

TODO

## Bool

Boolean is the result type of boolean expressions which may just be `true` or `false`.

[![test](.test/types_primitive_bool.png)](.test/types_primitive_bool.log)

```µcad,types_primitive_bool
std::debug::assert( true != false );
```
