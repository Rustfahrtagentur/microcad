# Orient `std::ops::orient`

Orients an object along a specified axis:

[![test](.test/orient_3d.svg)](.test/orient_3d.log)

```µcad,orient_3d
use std::math::*;
use std::ops::*;
use std::geo3d::*;

cylinder(h = 50mm, d = 35mm).orient([X,Y,Z]);
```
