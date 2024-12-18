# Use statement

## No use statement

```µcad,without_use
std::geo3d::sphere(radius = 40.0mm);
```

## Simple `use` statement

```µcad,with_use
use std::geo3d::sphere, std::geo3d::cube;

sphere(r = 4mm);
cube(size = 40.0mm);
```

## `use *` statement

```µcad,use_all_from
use std::geo3d::*;

cube(size = 40.0mm);
```

## `use as` statement

```µcad,use_as
use std::geo3d::sphere as ball;

ball(r = 40mm);
std::geo3d::sphere(r = 40mm);
```

## example

```µcad,example.A
// Use statement: sub-module `cube` from module `geo3d`.
use std::geo3d::cube;
use std::geo3d::sphere, std::geo3d::torus;

cube(size = 40mm); // calls geo3d.cube(size = 40mm);

std::geo3d::cone(height = 2cm);
```

Notice that the `size` parameter name is optional an can be omitted.
We need to export the cube as an STL file.
