# Hull

In the following examples the convex hull of circles is calculated.

[![test](.test/hull_single.png)](.test/hull_single.log)

```µcad,hull_single#todo
hull()
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
```

[![test](.test/hull_multiple.png)](.test/hull_multiple.log)

```µcad,hull_multiple#todo
use std::geo2d::*;
use std::algorithm::*;

hull() {
    union() {
        translate(x = [-10, 10]mm, y = [-10, 10]mm)
            circle(1mm);
        translate(x = [-20, 20]mm, y = 0mm)
            circle(1mm);
    }
}
```
