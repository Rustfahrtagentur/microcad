# µcad logo

[![test](.test/logo.svg)](.test/logo.log)

```µcad,logo
use std::geo2d::*;
use std::ops::*;

sketch icon_part( radius: Length ) {
    c = Circle(radius) - Rect(size = radius * 2).translate(y = radius );
    r = Rect(width = radius, height = radius * 2);
    c.translate(y = radius) | r.translate(x = -radius/2 );
}

sketch icon( radius = 1cm ) {
    gap = radius / 5;
    icon_part(radius)
        .translate(x = -radius-gap, y = 0mm, z = 0mm)
        .rotate(z = [0°, -90°, -180°, -270°]); 
}

icon(1cm);
```

![test](.test/logo-out.svg)
