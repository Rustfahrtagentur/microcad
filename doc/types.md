# µcad Types

## Primitive Types

### Integer

The type `integer` contains a natural number.

```µcad,primitive.integer
i = 3;
```

### Scalar

The type `scalar` contains a floating number and must be written with at least one decimal place (or in percent).

```µcad,primitive.scalar
zero = 0.0;
pi =  3.1415;
percent = 55%;
```

### Length

Length are used in describing dimensions and must be given with a unit.

```µcad,primitive.length
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37007874015748in;

std::assert( [millimeters, centimeters, meters, inches].equal() );
```

### Angle

Angles are used with rotations and in constrains when proving measures.

```µcad,primitive.angle
pi = std::math::pi;
radian = 1rad * pi;
degree = 180°;
degree_ = 180deg;
gradian = 200grad;
turn = 0.5turn;

std::assert( [degree, degree_, gradian, turn, radian].equal() );
```

### Area

```µcad,primitive.area#todo
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in²;

std::assert(square_millimeter = 0.1m²);
std::assert(square_centimeter = 0.1m²);
std::assert(square_inch ~ 0.1m² +-1%);
```

### Volume

```µcad,primitive.volume#todo
cubic_millimeter = 1000000mm³;
cubic_centimeter = 100cl;
cubic_meter = 0.001m³;
cubic_inch = 61.0237in³;
liter = 1l;
centiliter = 100cl;
milliliter = 1000ml;

std::assert(cubic_millimeter = 1l);
std::assert(cubic_centimeter = 1l);
std::assert(cubic_meter = 1l);
std::assert(cubic_inch ~ 1l +-1%);
std::assert(centiliter = 1l);
std::assert(milliliter = 1l);
```

### String

Text can be used to logging or to render text.

```µcad,primitive.string#todo
text = "Hello µcad!";
std::assert(text.count(11));

// logging
std::info(text);

// render text
std::geo2d::text(text);
```

### Color

Colors are defined by using hash mark (`#`) followed by hexadecimal digits for red, green and blue:

```µcad,primitive.color#todo
rgb_single_hex = #FFF;
rgb_double_hex = #00FF00;
rgba_single_hex = #FFFF;
rgba_double_hex = #00FF00FF;

std::assert( rgb_single_hex = rgba_single_hex );
std::assert( rgb_double_hex = rgba_double_hex );
```

Illegal values for color:

```µcad,primitive.no_color#fail
no_color = #00FF0
```

### Weight

Weights can be calculated by applying volumes to materials.

```µcad,primitive.weight#todo
gram = 1000g;
kilogram = 1kg;
pound = 2.204623lb;

std::assert(gram = 1kg);
std::assert(pound ~ 1kg +-1%);
```

### Vec2

2D vectors are written as named tuples with builtin values `x` and `y`.

```µcad,primitive.vec2
vec2 = (x=1, y=2)cm;
```

### Vec3

3D vectors are written as named tuples with builtin values `x`, `y` and `z`.

```µcad,primitive.vec3
vec3 = (x=1, y=2, z=3)cm;
```

### Vec4

TODO

### Bool

Boolean is the result type of boolean expressions which may just be `true` or `false`.

```µcad,primitive.bool
std::assert( true != false );
```
