# Union operation

We can combine the frame and the struts into a single geometry by using the
[`union`](../libs/std/ops/union.md)
operation or the `|` operator.

The code in the `lego_brick.µcad` with the whole 2D geometry of the brick's base will look like this:

[![test](.test/operations.svg)](.test/operations.log)

```µcad,operations
use std::geo2d::*;
use std::ops::*;

wall_width = 1.2mm;
width = 31.8mm;
height = 15.8mm;
frame = Frame(width, height, thickness = wall_width);
struts = Ring(outer_d = 6.51mm, inner_d = 4.8mm)
             .translate(x = [-1..2] * 8mm);

frame | struts;
```

If you export the file, you will see a frame and the structs combined into a single object.

![Picture](.test/operations-out.svg)
