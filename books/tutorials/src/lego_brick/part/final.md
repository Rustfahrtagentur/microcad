# The final part

Additionally, to the `grid` operation, we have to compute
the overall and width and height in the `LegoBrick` part:

* `width = rows * 8mm - 0.2mm`
* `height = columns * 8mm - 0.2mm`

Now we can write the final part of a `LegoBrick`.
We add some default values for `rows` and `columns` in the building plan and use
them in the last statement where we call `LegoBrick()`.

[![test](.test/final.svg)](.test/final.log)

```µcad,final
use std::ops::*;
use std::geo2d::*;

const SPACING = 8mm;

op grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .center()
}

sketch Base(rows: Integer, columns: Integer, width: Length, height: Length) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer = 6.51mm, inner = 4.8mm)
        .grid(rows = rows - 1, columns = columns - 1);
    frame | struts;
}

use Rect as Cap;

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm).grid(rows, columns);
}

part LegoBrick(rows = 4, columns = 2, base_height = 9.6mm) {
    width = rows * SPACING - 0.2mm;
    height = columns * SPACING - 0.2mm;
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

![Picture](.test/final-out.svg)

Let's make a library out of it and use it from another file in the next section.
