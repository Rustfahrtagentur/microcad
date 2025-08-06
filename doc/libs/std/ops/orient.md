# Orient `std::ops::orient`

Orients an object along a specified axis:

[![test](.test/orient_3d.png)](.test/orient_3d.log)

```µcad,orient_3d
use std::math::*;
use std::ops::*;
use std::geo3d::*;

orient([X,Y,Z]) cylinder(h = 50mm, d = 35mm);
```
