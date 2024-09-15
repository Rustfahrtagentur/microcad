# µCAD Types

## Primitive Types

### Integer

The type `integer` contains a natural number.

```µCAD,primitive.integer
i = 3;
```

### Scalar

The type `scalar` contains a floating number and must be written with at least one decimal place (or in percent).

```µCAD,primitive.scalar
zero = 0.0;
pi =  3.1415;
percent = 55%;
```

### Length

Length are used in describing dimensions and must be given with a unit.

```µCAD,primitive.length
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37008in;

assert( millimeters == centimeters == meters == inches );
```

### Angle

Angles are used with rotations and in constrains when proving measures.

```µCAD,primitive.angle
pi = PI;
radian = 1rad;
degree = 180°;
degree_ = 180deg;
gradient = 180grad;

assert( radian = PI );
assert( degree = PI );
assert( degree = PI );
assert( gradient = PI );
```

### Area

```µCAD,primitive.area
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in³;

assert(square_millimeter = 0.1m²);
assert(square_centimeter = 0.1m²);
assert(square_inch ~ 0.1m² +-1%);
```

### Volume

```µCAD,primitive.volume
cubic_millimeter = 1000000mm³;
cubic_centimeter = 100cl;
cubic_meter = 0.001m³;
cubic_inch = 61.0237in³;
liter = 1l;
centiliter = 100cl;
milliliter = 1000ml;

assert(cubic_millimeter = 1l);
assert(cubic_centimeter = 1l);
assert(cubic_meter = 1l);
assert(cubic_inch ~ 1l +-1%);
assert(centiliter = 1l);
assert(milliliter = 1l);
```

### String

Text can be used to logging or to render text.

```µCAD,primitive.string
text = "Hello µCAD!";
assert(text.count(11));

// logging
info(text);

// render text
std::geo2d::string(text);
```

### Color

Colors are defined by using hash mark (`#`) followed by hexadecimal digits for red, green and blue:

```µCAD,primitive.color
RGB_single_hex = #FFF
RGB_double_hex = #00FF00
RGBA_single_hex = #FFFF
RGBA_double_hex = #00FF00FF
```

Illegal values for color:

```µCAD,primitive.no_color#fail
no_color = #00FF0
```

### Weight

Weights can be calculated by applying volumes to materials.

```µCAD,primitive.weight
gram = 1000g;
kilogram = 1kg;
pound = 2.204623lb;

assert(gram = 1kg);
assert(pound ~ 1kg +-1%);
```

### Vec2

```µCAD,primitive.vec2
```

### Vec3

```µCAD,primitive.vec3
```

### Vec4

```µCAD,primitive.vec4
```

### Bool

Boolean is the result type of boolean expressions.

```µCAD,primitive.bool
t = true;
f = false;

assert( t != f );
```
