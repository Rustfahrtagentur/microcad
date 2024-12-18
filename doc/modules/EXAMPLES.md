# Examples

## Parametric Module

* Parametric modules have a parameter lists

### Calculation in Function

```µcad,functions#todo
module cube_with_volume(size: length) {

    function volume() {
        size*size*size
    }

    function weight(density: density = 20g/1mm^3) {
        volume() * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume()}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```

### Calculation in Field Initialization

```µcad,fields#todo
module cube_with_volume(size: length) {

    volume = size*size*size;

    function weight(density: density = 20g/1mm^3) {
        volume * density
    }

    cube(size);
}

my_cube = cube_with_volume(40mm);
info("Cube volume: {my_cube.volume}");
info("Cube weight: {my_cube.weight(40g/mm^3)}");
```
