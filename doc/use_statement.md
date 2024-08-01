# Use statement

## No use statement

```ucad
geo3d::sphere(4mm);
geo3d::torus(r1 = 10mm, r2 = 2mm);
```

## Simple `use` statement

```ucad
use geo3d::sphere, geo3d::torus;

sphere(4mm);
torus(r1 = 10mm, r2 = 2mm);
```

## `use from` statement

```ucad
use sphere, torus from geo3d;

sphere(4mm);
torus(r1 = 10mm, r2 = 2mm);
```

## `use *` statement

```
use * from geo3d;


```

## example

```ucad
// Use statement: sub-module `cube` from module `geo3d`.
use cube from geo3d;
use sphere, torus from geo3d;

cube(size = 40mm); // calls geo3d.cube(size = 40mm);

geo3d.cone(height = 2cm);
```

Notice that the `size` parameter name is optional an can be omitted.
We need to export the cube as an STL file.
