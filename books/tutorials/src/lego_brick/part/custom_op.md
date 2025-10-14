# Custom operation

The knobs and struts are created using multiplicity by `translate` and `align` operations.

To make placing the elements more generic we will create an operation called `grid`
which arranges elements in a grid which is centered to origin:

[![test](.test/grid.svg)](.test/grid.log)

```µcad,grid
use std::ops::*;

const SPACING = 8mm;

op grid(rows: Integer, columns: Integer) {
    @input
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .align()
}
```

The `grid` operation takes `rows` and `columns` as parameters.

Operations - as we already know - have not only an output geometry but an input geometry as well.
To be able to access those input geometry we need to use the keyword `@input`.
With `@input` we insert the elements that are given by the caller.
In our case that will be a knob or a strut sketch.

We now can rewrite `Knobs` and `Frame` sketches by adding `rows` and `columns`
as parameter and using the `grid` operation:

[![test](.test/custom_op.svg)](.test/custom_op.log)

```µcad,custom_op
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

sketch Knobs(columns: Integer, rows: Integer) {
    Circle(d = 4.8mm)
        .grid(columns, rows);
}

use Rect as Cap;

columns = 4;
rows = 2;
width = columns * SPACING - 0.2mm;
height = rows * SPACING - 0.2mm;

Base(rows, columns, width, height);
Cap(width, height);
Knobs(rows, columns);
```

![Picture](.test/custom_op-out.svg)

Additionally, to the `grid` operation, we compute the overall width and
height in the `LegoBrick` part, which are:

* `width = rows * 8mm - 0.2mm`
* `height = columns * 8mm - 0.2mm`

Now we are ready to write the final part of a `LegoBrick`.
