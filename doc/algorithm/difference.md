# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

```µcad,difference.operator
std::geo2d::circle(radius = 3.0mm) - std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Difference module

```µcad,difference.module
use std::*;

algorithm::difference() {
    geo2d::circle(radius = 3.0mm);
    geo2d::rect(width = 3.0mm, height = 2.0mm);
};
```
