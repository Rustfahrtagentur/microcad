# Examples

## Parametric Module

* Parametric modules have a parameter lists

### Calculation in Function

```µCAD,functions
module cube_with_volume(size: length) {

    function volume() {
        size*size*size
    }

    function weight(density: weight / length^3 = 20g/1mm^3) {
        volume() * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume()}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```

### Calculation in Field Initialization

```µCAD,fields
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
