# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

![test](.test/difference_operator.png)

```µcad,difference_operator
std::geo2d::circle(radius = 3.0mm) - std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Difference module

![test](.test/difference_module.png)

```µcad,difference_module
use std::*;

algorithm::difference() {
    geo2d::circle(radius = 3.0mm);
    geo2d::rect(width = 3.0mm, height = 2.0mm);
};
```
