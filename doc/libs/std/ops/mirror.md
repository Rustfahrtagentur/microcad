# Orient `std::ops::mirror`

Mirror an object along an axis:

[![test](.test/mirror_3d.png)](.test/mirror_3d.log)

```µcad,mirror_3d
use std::math::*;
use std::ops::*;
use std::geo3d::*;

s = 25mm;
p = 5mm;
{ 
    (Cube(s * 2) - Cube(s).translate(x = s/2, y = s/2, z = s/2))
        .translate(x = s + p, y = s + p, z = s + p)
}
//.mirror([X,Y,Z]);
```
