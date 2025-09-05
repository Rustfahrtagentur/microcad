# Custom operation

The knobs and struts are created using multiplicity and the knobs already use the `align()`
operation and the struts might use this too.

To make placing the elements more generic we will create an operation called `grid`
which arranges elements in a grid which is centered to origin:

[![test](.test/grid.svg)](.test/grid.log)

```µcad,grid
const SPACING = 8mm;

op grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .align()
}
```

The `grid` operation takes `rows` and `columns` as parameters.

Operations - as we already know - have not only an output geometry but an input geometry as well.
To be able to access those input geometry we need to use the keyword `@children`.
With `@children` we insert the elements that are given by the caller.
In our case that will be a knob or a strut sketch.

We now can rewrite `Knobs` and `Frame` sketches by adding `rows` and `columns`
as parameter and using the `grid` operation:

[![test](.test/custom_op.svg)](.test/custom_op.log)

```µcad,custom_op
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

op grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .align()
}

sketch Base(
    rows: Integer,
    columns: Integer,
    width: Length,
    height: Length
) {
    thickness = 1.2mm;
    frame = Frame(width, height, thickness);
    struts = Ring(outer = 6.51mm, inner = 4.8mm)
        .grid(rows, columns);
    frame | struts;
}

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm)
        .grid(rows, columns);
}
```

![Picture](.test/custom_op-out.svg)
