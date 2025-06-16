# Quantity

*Quantities* are numeric values coupled with a unit.
Each unit refers to example one quantity type.

The following quantity types are supported:

| Type      | Metric Units                                | Imperial Units                 |
| --------- | ------------------------------------------- | ------------------------------ |
| `Scalar`  | -, `%`                                      | -                              |
| `Length`  | `µm`, `mm`, `cm`, `m`                       | `in` or `"`, `ft` or `'`, `yd` |
| `Angle`   | `°` or `deg`, `grad`, `turn`,`rad`          |                                |
| `Weight`  | `g`, `kg`                                   | `lb`, `oz`                     |
| `Area`    | `µm²`,`mm²`,`cm²`,`m³`                      | `in²`, `ft²` , `yd²`           |
| `Volume`  | `µm³`, `mm³`,`cm³`,`m³`,`ml`,`cl`,`l`, `µl` | `in³`, `ft³` , `yd³`           |

**Note**: More units [may be implemented](https://github.com/Rustfahrtagentur/microcad/issues/76).

## Literals

Quantities can be declared by *literals*.
This means that your will automatically get the following type if you use the beside units:

[![test](.test/quantity_types_number_literals.png)](.test/quantity_types_number_literals.log)

```µcad,quantity_types_number_literals
// declare variable `height` of type `Length` to 1.4 Meters
height = 1.4m;

// use as *default* value in parameter list
fn f( height = 1m ) {}

// calculate a `Length` called `width` by multiplying the
// `height` with `Scalar` `2` and add ten centimeters
width = height * 2 + 10cm;
```

## Examples

### Scalar

The type `Scalar` contains a floating number and must be written with at least one decimal place (or in percent).

[![test](.test/types_quantity_scalar.png)](.test/types_quantity_scalar.log)

```µcad,types_quantity_scalar
zero = 0;
pi =  3.1415;
percent = 55%;
```

### Length

`Length` is used to describe a one-dimensional quantity.

[![test](.test/types_quantity_length.png)](.test/types_quantity_length.log)

```µcad,types_quantity_length
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37007874015748in;

std::debug::assert( [millimeters, centimeters, meters, inches].all_equal() );
```

### Angle

Angles are used with rotations and in constrains when proving measures.

[![test](.test/types_quantity_angle.png)](.test/types_quantity_angle.log)

```µcad,types_quantity_angle
pi = std::math::PI;
radian = 1rad * pi;
degree = 180°;
degree_ = 180deg;
grad = 200grad;
turn = 0.5turn;

std::debug::assert( [degree, degree_, grad, turn, radian].all_equal() );
```

### Area

An `Area` is a two-dimensional quantity. It is the result when multiplying two `Length`.

[![test](.test/types_quantity_area.png)](.test/types_quantity_area.log)

```µcad,types_quantity_area
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in²;

std::debug::assert(square_millimeter == 0.1m²);
std::debug::assert(square_centimeter == 0.1m²);
```

## Volume

A `Volume` is a three-dimensional quantity. It is the result when multiplying three `Length`.

[![test](.test/types_quantity_volume.png)](.test/types_quantity_volume.log)

```µcad,types_quantity_volume
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

## Weight

Weights can be calculated by applying volumes to materials.

[![test](.test/types_quantity_weight.png)](.test/types_quantity_weight.log)

```µcad,types_quantity_weight
gram = 1000.0g;
kilogram = 1.0kg;
pound = 2.204623lb;

std::debug::assert(gram == 1.0kg);
```