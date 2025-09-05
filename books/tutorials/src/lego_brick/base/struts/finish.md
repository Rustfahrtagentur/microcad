# Finish the struts

Maybe you already question yourself if there is something similar to `std::geo2d::Frame()` for our circles?

And yes there is, and it's called `std::geo2d::Ring`.
So let's shorten our strut code a last time:

[![test](.test/ring.svg)](.test/ring.log)

```µcad,ring
use std::geo2d::*;
use std::ops::*;

Ring(outer_d = 6.51mm, inner_d = 4.8mm).translate(x = [-1..1] * 8mm);
```

![Picture](.test/ring-out.svg)