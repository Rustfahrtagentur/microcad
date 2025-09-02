# Putting it together

Below is an intermediate result of the sketches of the three components we now have defined:

```µcad,tutorial_part_overview
use std::geo2d::*;
use std::ops::*;

spacing = 8mm;

sketch Base(width: Length, height: Length) {
    wall_width = 1.2mm;
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .translate(x = [0..3] * spacing)
        .center();
    frame | struts;
}

use Rect as Cap;

sketch Knobs() {
    Circle(d = 4.8mm, center = (x = [0..3] * spacing, y = [0..1] * spacing)).center();
}

width = 15.8mm;
height = 31.8mm;

Base(width, height);
Cap(width, height);
Knobs();
```

In the next steps, we want to create a 3D geometry.
