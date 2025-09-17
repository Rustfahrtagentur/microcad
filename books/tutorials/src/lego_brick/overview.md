# Putting it together

Below is an intermediate result of the sketches of the three components that have now been defined:

[![test](.test/overview.svg)](.test/overview.log)

```µcad,overview
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer_d = 6.51mm, inner_d = 4.8mm)
        .translate(x = [0..2] * SPACING)
        .align();
    frame | struts;
}

use Rect as Cap;

sketch Knobs() {
    Circle(d = 4.8mm, c = (x = [0..3] * SPACING, y = [0..1] * SPACING))
        .align();
}

width = 31.8mm;
height = 15.8mm;

Base(width, height);
Cap(width, height);
Knobs();
```

![Picture](.test/overview-out.svg)

Across the Lego universe, the spacing of `8mm` is used everywhere.
To address this we can store it in a *constant* named `SPACING` using the `const` keyword.
This makes `SPACING` available in all sketches.
The name of a constant must be in capital letters.
It can be used from within the current module/file and from all sketches and parts within that module.

In the next steps, we want to create a 3D geometry.
