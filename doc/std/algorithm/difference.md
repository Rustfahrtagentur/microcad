# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

[![test](.test/difference_operator.png)](.test/difference_operator.log)

```µcad,difference_operator
std::geo2d::circle(radius = 3mm) - std::geo2d::rect(width = 3mm, height = 2mm);
```

## Difference module

[![test](.test/difference_module.png)](.test/difference_module.log)

```µcad,difference_module
use std::*;

algorithm::difference() {
    geo2d::circle(radius = 3mm);
    geo2d::rect(width = 3mm, height = 2mm);
};
```
