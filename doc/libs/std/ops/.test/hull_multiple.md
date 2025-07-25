# Test [`hull_multiple`](/doc/libs/std/ops/hull.md#L18)

## Code

```µcad
use std::geo2d::*;
use std::ops::*;

hull() {
    union() {
        translate(x = [-10, 10]mm, y = [-10, 10]mm)
            circle(1mm);
        translate(x = [-20, 20]mm, y = 0mm)
            circle(1mm);
    }
}

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/libs/std/ops/.test/hull_multiple.png)
