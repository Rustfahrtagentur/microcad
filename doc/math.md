# Math

## Builtin functions

### Absolute Value

Calculate absolute value:

```µCAD,abs
use std::debug::assert;
use std::math::abs;

assert(abs(-1.0) == 1);
```

### Trigonometric functions

```µCAD,trigonometric
use std::debug::assert;
use * from std::math;

assert(sin(PI) == 0.0);
assert(cos(PI) == -1.0);
assert(tan(PI) == 0.0);
assert(cot(PI/2.0) == 0.0);
assert(sin(x)^2 + cos(x)^2 == 1.0);
```

### PI

```µCAD,pi
use std::debug::assert;
use std::math::PI;

assert(PI ~ 3.14 +-0.01);
```
