# Modules

## Syntax

`module` *name* `(`*parameter_list*`)` `{`
    *use_statement* |
    *expression_statement* |
    *assignment_statement* |
    *module_init_definition* |
    *module_definition* |
    *function_definition*
`}`

### Basic Example

A 2D donut as circle with a hole.

```µCAD,basic_example
// load module circle from module geo2d
use circle from geo2d;

// define module donut with two parameters
module donut(r_outer: length, r_inner: length) {
    // generate donut which is the difference from two circles
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

## Namespace module

* Provides function and modules
* No parameter list

```µcad
module math {
}

module algorithm {
    module union() {
        init() {
            
        }
    }
}

```

## Parametric module

* Parametric modules have a parameter lists

### Member functions

```µcad
module cube_with_volume(size: length) {

    function volume() {
        size*size*size
    }

    function weight(density: weight / length^3 = 20g/mm^3) {
        volume() * density
    }

    cube(size);
}
```

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume()}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");

### Member variable with initialization

```µcad
module cube_with_volume(size: length) {

    volume = size*size*size;
    volume = math::volume(size);

    function weight(density: weight / length^3 = 20g/mm^3) {
        volume * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```

### Member function with parameters

module cube_with_volume(size: length) {