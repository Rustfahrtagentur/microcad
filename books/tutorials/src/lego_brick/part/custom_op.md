# Custom operation

The knobs and struts are created using multiplicity and the knobs already use the `center()`
operation and the struts might use this too.

To make placing the elements more generic we will create an operation called `center_grid`
which arranges elements in a grid which is centered to origin:

```µcad
spacing = 8mm;

op center_grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * spacing, y = [0..columns] * spacing)
        .center()
}
```

The `center_grid` operation takes `rows` and `columns` as parameters.

Operations - as we already know - have not only an output geometry but an input geometry as well.
To be able to access those input geometry we need to use the keyword `@children`.
With `@children` we insert the elements that are given by the caller.
In our case that will be a knob or a strut sketch.

We now can rewrite `Knobs` and `Frame` sketches by adding `rows` and `columns`
as parameter and using the `center_grid` operation:

```µcad
use std::geo2d::*;
use std::ops::*;

spacing = 8mm;

op center_grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * spacing, y = [0..columns] * spacing)
        .center()
}

sketch Base(
    rows: Integer,
    columns: Integer,
    width: Length,
    height: Length
) {
    thickness = 1.2mm,
    frame = Frame(width, height, thickness);
    struts = (Ring(outer = 6.51mm, inner = 4.8mm)).center_grid(rows, columns);
    frame | struts;
}

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm).center_grid(rows, columns);
}
```
