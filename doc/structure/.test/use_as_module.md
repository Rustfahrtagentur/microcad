# Test [`use_as_module`](/doc/structure/use.md#L72)

## Code

```µcad
geo2d = 1;

use std::geo2d as geo;

geo::circle(r = 4mm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/use_as_module.png)
