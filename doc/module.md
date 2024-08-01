# Modules

## Declaration

### Donut example

A donut as circle with a hole.

```µCAD
module donut(r_outer: length, r_inner: length) {
    circle(r_outer) - circle(r_inner);
}
```

## Declaration

```µcad
// We have to import the primitive2d module to use `hexagon` and `circle` sub-modules
use * from geo2d;

// A generic module for the hex nut
module hex_nut(outer_diameter: length, hole_diameter: length) {
    hexagon(d = outer_diameter) - circle(d = hole_diameter);
}


## Initializers

## Member fields

## Methods
