# Use statement

## No use statement

![test](.banner/use_statement_without_use.png)

```µcad,use_statement_without_use
std::geo3d::sphere(radius = 40mm);
```

## Simple `use` statement

![test](.banner/use_statement_with_use.png)

```µcad,use_statement_with_use
use std::geo3d::sphere, std::geo3d::cube;

sphere(r = 4mm);
cube(size = 40mm);
```

## `use *` statement

![test](.banner/use_statement_use_all_from.png)

```µcad,use_statement_use_all_from
use std::geo3d::*;

cube(size = 40mm);
```

## `use as` statement

![test](.banner/use_statement_use_as.png)

```µcad,use_statement_use_as
use std::geo3d::sphere as ball;

ball(r = 40mm);
std::geo3d::sphere(r = 40mm);
```

## example

![test](.banner/use_statement_example_A.png)

```µcad,use_statement_example_A#todo
// Use statement: sub-module `cube` from module `geo3d`.
use std::geo3d::cube;
use std::geo3d::sphere, std::geo3d::torus;

cube(size = 40mm); // calls geo3d.cube(size = 40mm);

std::geo3d::cone(height = 2cm);
```

Notice that the `size` parameter name is optional an can be omitted.
We need to export the cube as an STL file.
