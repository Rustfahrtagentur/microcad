# Hull

In the following examples the convex hull of circles is calculated.

[![test](.test/hull_single.svg)](.test/hull_single.log)

```µcad,hull_single
use std::geo2d::*;
use std::ops::*;

{
    Circle(10mm).translate(x = 10mm, y =  0mm);
    Circle(10mm).translate(x =  0mm, y = 10mm);
}.hull();
```

[![test](.test/hull_multiple.svg)](.test/hull_multiple.log)

```µcad,hull_multiple
use std::geo2d::*;
use std::ops::*;

Circle(r = 4mm)
    .translate(x = [-10, 10]mm, y = [-10, 10]mm)
    .hull();
```
