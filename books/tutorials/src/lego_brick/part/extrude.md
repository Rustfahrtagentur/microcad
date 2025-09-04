# Extrude into 3D

Now we want to convert the three 2D sketches into a 3D geometry part.
This can be achieved by *extrusion*.
The corresponding µcad operation is called
[`std::ops::extrude`](../libs/std/ops/extrude.md).

As a first example, let's take the cap of brick and extrude with a height of 1.0 mm.

[![test](.test/extrude_cap.svg)](.test/extrude_cap.log)

```µcad,extrude_cap
use std::ops::extrude;
use std::geo2d::Rect as Cap;

width = 15.8mm;
height = 31.8mm;

Cap(width, height)
    .extrude(1.0mm);
```

This will create a box with dimensions 15.8 ⨯ 31.8 ⨯ 1.0 mm.

Notice that with `std::ops::extrude()` we always will extrude along Z axis.

![Picture](.test/extrude_cap-out.svg)
