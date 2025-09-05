# Create a first version of the LogoBrick

Let's make a brick out of our `Base`, the `Knobs` and the `Cap` sketch and integrate everything into a part.

We extrude `Base`, `Knobs` and `Cap` and translate it in Z direction if necessary.
Afterwards, we combine the three components by the `|` operator.

[![test](.test/first_version.svg)](.test/first_version.log)

```µcad,first_version
use std::ops::*;
use std::geo2d::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer_d = 6.51mm, inner_d = 4.8mm)
        .translate(y = [0..2] * SPACING)
        .align();
    frame | struts;
}

use Rect as Cap;

sketch Knobs() {
    center = (x = [0..3] * SPACING, y = [0..1] * SPACING);
    Circle(diameter = 4.8mm, center)
        .align();
}

part LegoBrick(base_height = 9.6mm) {
    width = 15.8mm;
    height = 31.8mm;
    top_thickness = 1.0mm;

    base = Base(width, height)
        .extrude(base_height);

    cap = Cap(width, height)
        .extrude(top_thickness)
        .translate(z = base_height - top_thickness);

    knobs = Knobs()
        .extrude(1.7mm)
        .translate(z = base_height);

    // Combine all components
    base | cap | knobs;
}

LegoBrick();
```

When we take the code snippet above and export it, instead of an SVG, an STL file will be exported:

![Picture](.test/first_version-out.svg)
