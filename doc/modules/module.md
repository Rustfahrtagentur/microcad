# Modules

## Declaration

```µCAD,declaration
// define custom module circle
module circle() {
    // generate circle by using module geo2d
    std::geo2d::circle(1cm);
}

// generate module circle
circle();
```

## Module Elements

### Module Attributes

TODO

### Module Parameter List

A 2D donut as circle with a hole.

```µCAD,parameters
// declare two parameters
module donut(outer: length, inner: length) {
    // parameters can be used anywhere within the module
    std::geo2d::circle(outer) - std::geo2d::circle(inner);
}

// generate donut of specific size
donut(2cm,1cm);
```

### Module Use

```µCAD,use_statement
module donut(outer: length, inner: length) {
    // load circle from module geo2d
    use circle from std::geo2d;

    // circle is now without geo2d prefix
    circle(outer) - circle(inner);
}

donut(2cm,1cm);
```

### Module Initialization

```µCAD,initialization
module donut(radius_outer: length, radius_inner: length) {
    use circle from std::geo2d;

    // alternative initialization with diameters
    init( diameter_outer: length, diameter_inner: length ) {
        // calculate radiuses from diameters
        radius_inner = diameter_inner/2;
        radius_outer = diameter_outer/2;
    }

    // generate donut based on radiuses
    circle(radius_outer) - circle(radius_inner);
}

// generate three equal donuts
donut( 2cm, 1cm );
donut( radius_outer=2cm, radius_inner=1cm );
donut( diameter_outer=4cm, diameter_inner=2cm );
```

### Module Fields

```µCAD,member.fields
module donut(radius) {
    use circle from std::geo2d;

    // calculate inner from radius
    inner = radius/2;

    // generate donut
    circle(radius) - circle(inner);
}
```

### Module Functions

```µCAD,member.functions
module donut(radius) {
    use circle from std::geo2d;

#    // calculate inner from radius in a method
    function inner() { radius/2 }

    // generate donut
    circle(radius) - circle(inner());
}
```

## Usage Examples

### Parametric Module

* Parametric modules have a parameter lists

#### Calculation in Function

```µcad,examples.parametric_module.functions
module cube_with_volume(size: length) {

    function volume() {
        size*size*size
    }

    function weight(density: weight / length^3 = 20g/mm^3) {
        volume() * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume()}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```

#### Calculation in Field Initialization

```µcad,examples.parametric_module.fields
module cube_with_volume(size: length) {

    volume = size*size*size;

    function weight(density: weight / length^3 = 20g/mm^3) {
        volume * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```
