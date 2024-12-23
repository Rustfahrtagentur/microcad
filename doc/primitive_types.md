# Primitive Types

## Integer

The type `integer` contains a natural number.

![test](.banner/types_primitive_integer.png)

```µcad,types_primitive_integer
i = 3;
```

## Scalar

The type `Scalar` contains a floating number and must be written with at least one decimal place (or in percent).

![test](.banner/types_primitive_scalar.png)

```µcad,types_primitive_scalar
zero = 0.0;
pi =  3.1415;
percent = 55%;
```

## Length

Length are used in describing dimensions and must be given with a unit.

![test](.banner/types_primitive_length.png)

```µcad,types_primitive_length
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37007874015748in;

std::assert( [millimeters, centimeters, meters, inches].equal() );
```

## Angle

Angles are used with rotations and in constrains when proving measures.

![test](.banner/types_primitive_angle.png)

```µcad,types_primitive_angle
pi = std::math::pi;
radian = 1rad * pi;
degree = 180°;
degree_ = 180deg;
grad = 200grad;
turn = 0.5turn;

std::assert( [degree, degree_, grad, turn, radian].equal() );
```

## Area

![test](.banner/types_primitive_area.png)

```µcad,types_primitive_area
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in²;

std::assert(square_millimeter == 0.1m²);
std::assert(square_centimeter == 0.1m²);
```

## Volume

![test](.banner/types_primitive_volume.png)

```µcad,types_primitive_volume
cubic_millimeter = 1000000.0mm³;
cubic_centimeter = 100.0cl;
cubic_meter = 0.001m³;
cubic_inch = 61.0237in³;
liter = 1.0l;
centiliter = 100.0cl;
milliliter = 1000.0ml;

std::assert(cubic_millimeter == 1.0l);
std::assert(cubic_centimeter == 1.0l);
std::assert(cubic_meter == 1.0l);
std::assert(centiliter == 1.0l);
std::assert(milliliter == 1.0l);
```

## String

Text can be used to logging or to render text.

![test](.banner/types_primitive_string.png)

```µcad,types_primitive_string#todo
text = "Hello µcad!";
std::assert(text.count(11));

// logging
std::info(text);

// render text
std::geo2d::text(text);
```

## Color

Colors are defined by using hash mark (`#`) followed by hexadecimal digits for red, green and blue:

![test](.banner/types_primitive_color.png)

```µcad,types_primitive_color
rgb_single_hex = #FFF;
rgb_double_hex = #00FF00;
rgba_single_hex = #FFFF;
rgba_double_hex = #00FF00FF;

std::assert( rgb_single_hex == rgba_single_hex );
std::assert( rgb_double_hex == rgba_double_hex );
```

Illegal values for color:

![test](.banner/types_primitive_no_color.png)

```µcad,types_primitive_no_color#fail
no_color = #00FF0
```

## Weight

Weights can be calculated by applying volumes to materials.

![test](.banner/types_primitive_weight.png)

```µcad,types_primitive_weight
gram = 1000.0g;
kilogram = 1.0kg;
pound = 2.204623lb;

std::assert(gram == 1.0kg);
```

### Vec2

2D vectors are written as named tuples with builtin values `x` and `y`.

![test](.banner/types_primitive_vec2.png)

```µcad,types_primitive_vec2
vec2 = (x=1, y=2)cm;
```

## Vec3

3D vectors are written as named tuples with builtin values `x`, `y` and `z`.

![test](.banner/types_primitive_vec3.png)

```µcad,types_primitive_vec3
vec3 = (x=1, y=2, z=3)cm;
```

## Vec4

TODO

## Bool

Boolean is the result type of boolean expressions which may just be `true` or `false`.

![test](.banner/types_primitive_bool.png)

```µcad,types_primitive_bool
std::assert( true != false );
```
