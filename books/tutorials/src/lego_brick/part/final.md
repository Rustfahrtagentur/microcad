# The final part

First we add some default values for `rows` and `columns` in the building plan and use
them in the last statement where we call `LegoBrick()`.

[![test](.test/final.svg)](.test/final.log)

```µcad,final(hires)
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

op grid(columns: Integer, rows: Integer) {
    @input
        .translate(x = [1..columns] * SPACING, y = [1..rows] * SPACING)
        .align()
}

sketch Base(
    columns: Integer,
    rows: Integer,
    width: Length,
    height: Length
) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer_d = 6.51mm, inner_d = 4.8mm)
        .grid(columns = columns-1, rows = rows-1);
    frame | struts;
}

use Rect as Cap;

sketch Knobs(columns: Integer, rows: Integer) {
    Circle(d = 4.8mm)
        .grid(columns, rows);
}

part LegoBrick(rows = 2, columns = 4, base_height = 9.6mm) {
    width = columns * SPACING - 0.2mm;
    height =rows * SPACING - 0.2mm;
    cap_thickness = 1.0mm;

    base = Base(rows, columns, width, height)
        .extrude(base_height - cap_thickness);

    cap = Cap(width, height)
        .extrude(cap_thickness)
        .translate(z = base_height - cap_thickness);

    knobs = Knobs(rows, columns)
        .extrude(1.7mm)
        .translate(z = base_height);

    base | cap | knobs;
}

// render a brick with default values
LegoBrick();
```

![Picture](final-out.png)

Let's make a library out of it, and use it from another file in the next section.
