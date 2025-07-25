# Test [`math_trigonometric`](/doc/libs/std/math/README.md#L19)

## Code

```µcad
use std::debug::*;
use std::math::*;

assert_eq([cos(PI), -1.]);
assert_eq([tan(0), 0.]);

x = 0.5;
assert_eq([sin(x)^2. + cos(x)^2., 1.]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/libs/std/math/.test/math_trigonometric.png)
