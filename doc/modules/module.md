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

## Declaration

A fixed circle.

```µCAD,declaration
// define custom module circle
module circle() {
    // generate circle
    geo2d::circle(1cm);
}

// generate module circle
circle();
```

## Parameter initialization

A 2D donut as circle with a hole.

```µCAD,init_parameters
// declare two parameters
module donut(outer: length, inner: length) {
    // parameters can used anywhere within the module
    geo2d::circle(outer) - geo2d::circle(inner);
}

// generate donut of specific size
donut(2cm,1cm);
```

## Use other modules more elegant

```µCAD,use
module donut(outer: length, inner: length) {
    // load circle from module geo2d
    use circle from geo2d;

    // circle is now without geo2d prefix
    circle(outer) - circle(inner);
}

donut(2cm,1cm);
```

## Alternative initializations

```µCAD,init_alternative
module donut(radius_outer: length, radius_inner: length) {
    // alternative initialization with diameters
    init( diameter_outer: length, diameter_inner: length ) {
        // calculate radiuses from diameters
        radius_inner = diameter_inner/2;
        radius_outer = diameter_outer/2;
    }
    // generate donut based on radiuses
    geo2d::circle(radius_outer) - geo2d::circle(radius_inner);
}

// generate three equal donuts
donut( 2cm, 1cm );
donut( radius_outer=2cm, radius_inner=1cm );
donut( diameter_outer=4cm, diameter_inner=2cm );
```

## Member fields

```µCAD,member_fields
module donut(radius) {
    inner = radius/2;
    geo2d::circle(radius) - geo2d::circle(inner);
}
```

## Methods

```µCAD,member_methods
module donut(radius) {
    function inner() { radius/2 }
    geo2d::circle(radius) - geo2d::circle(inner());
}
```

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
