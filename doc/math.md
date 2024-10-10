# Math

## Builtin functions

### Absolute Value

Calculate absolute value:

```µCAD,abs
std::assert(std::math::abs(-1.0) == 1.0);
```

### Trigonometric functions

```µCAD,trigonometric#todo
use std::assert;
use * from std::math;

assert(sin(PI) == 0.0);
assert(cos(PI) == -1.0);
assert(tan(PI) == 0.0);
assert(cot(PI/2.0) == 0.0);
assert(sin(x)^2 + cos(x)^2 == 1.0);
```

### PI

```µCAD,pi#todo
use std::assert;
use std::math::PI;

assert(PI ~ 3.14 +-0.01);
```
