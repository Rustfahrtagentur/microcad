# math

## Mathematical functions

### Absolute Value

Calculate absolute value:

![test](.test/math_abs.png)
[see build log](.test/math_abs.log)

```µcad,math_abs
std::assert(std::math::abs(-1.0) == 1.0);
```

### Trigonometric functions

![test](.test/math_trigonometric.png)
[see build log](.test/math_trigonometric.log)

```µcad,math_trigonometric#todo
use std::*;
use std::math::*;

assert(cos(pi) == -1.0);
assert(tan(0.0) == 0.0);
//assert(cot(pi/2.0) == 0.0);
//assert(sin(x)^2 + cos(x)^2 == 1.0);
```
