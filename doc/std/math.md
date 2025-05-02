# math

## Mathematical functions

### Absolute Value

Calculate absolute value:

[![test](.test/math_abs.png)](.test/math_abs.log)

```µcad,math_abs
std::debug::assert(std::math::abs(-1) == 1);
```

### Trigonometric functions

[![test](.test/math_trigonometric.png)](.test/math_trigonometric.log)

```µcad,math_trigonometric#todo
use std::debug::*;
use std::math::*;

assert(cos(pi) == -1.);
assert(tan(0) == 0.);
//assert(cot(pi/2) == 0.);
//assert(sin(x)^2. + cos(x)^2. == 1.);
```
