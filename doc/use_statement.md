# Use statement

## No use statement

```µCAD,without_use
std::geo3d::sphere(radius = 40.0mm);
```

## Simple `use` statement

```µCAD,with_use#todo
use std::geo3d::sphere, std::geo3d::torus;

sphere(4mm);
torus(r1 = 10mm, r2 = 2mm);
```

## `use from` statement

```µCAD,use_from#todo
use sphere, torus from geo3d;

sphere(4mm);
torus(r1 = 10mm, r2 = 2mm);
```

## `use *` statement

```µCAD,use_all_from
use std::geo3d::*;

cube(size = 40.0mm);
```

## `use as` statement

```µCAD,use_as#todo
use std::geo3d::sphere as ball;

ball(r = 40mm);
std::geo3d::sphere(r = 40mm);
```

## example

```µCAD,example.A
// Use statement: sub-module `cube` from module `geo3d`.
use std::geo3d::cube;
use std::geo3d::sphere, std::geo3d::torus;

cube(size = 40mm); // calls geo3d.cube(size = 40mm);

std::geo3d::cone(height = 2cm);
```

Notice that the `size` parameter name is optional an can be omitted.
We need to export the cube as an STL file.
