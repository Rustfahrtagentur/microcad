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

## Elements

* [Use Statements](use.md)
* [Parameter Lists](parameter_list.md)
* [Fields](fields.md)
* [Initialization](init.md)
* [Functions](functions.md)

## Examples

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
