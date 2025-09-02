# Extrude into 3D

Now we want to convert the three 2D sketches into a 3D geometry part.
This can be achieved by *extrusion* and the corresponding µcad operation is called
[`std::ops::extrude`](../libs/std/ops/extrude.md).

As a first example, let's take the cap of brick and extrude with a height of 1.0 mm.

```µcad,tutorial_part_extrude_cap
use std::ops::extrude;
use std::geo2d::Rect as Cap;

width = 15.8mm;
height = 31.8mm;

Cap(width, height)
    .extrude(z = 1.0mm);
```

This will create a box with dimensions 15.8 ⨯ 31.8 ⨯ 1.0 mm.

Notice that using `height = 1.0` will automatically extrude along Z axis.

![Picture]()
