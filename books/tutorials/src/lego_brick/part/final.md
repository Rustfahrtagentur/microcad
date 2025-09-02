# The final part

Additionally, to the `center_grid` operation, we have to compute
the overall and width and height in the `LegoBrick` part:

* `width = rows * 8mm - 0.2mm`
* `height = columns * 8mm - 0.2mm`

Now we can write the final part of a `LegoBrick`:

```µcad,final
use std::ops::*;
use std::geo2d::*;

spacing = 8mm;

op center_grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * spacing, y = [0..columns] * spacing)
        .center()
}

sketch Base(rows: Integer, columns: Integer, width: Length, height: Length) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer = 6.51mm, inner = 4.8mm)
        .center_grid(rows = rows - 1, columns = columns - 1);
    frame | struts;
}

use Rect as Cap;

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm).center_grid(rows, columns);
}

part LegoBrick(rows: Integer, columns: Integer, base_height = 9.6mm) {
    width = rows * spacing - 0.2mm;
    height = columns * spacing - 0.2mm;
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

// Let's create a few bricks with different parameters
LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2)
    .translate(y = -40mm);
LegoBrick(rows = 4, columns = 2);
LegoBrick(rows = 3, columns = 2, base_height = 3.2mm)
    .translate(y = 40mm);
```
