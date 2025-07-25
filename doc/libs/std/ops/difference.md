# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

[![test](.test/difference_operator.png)](.test/difference_operator.md)

```µcad,difference_operator
std::geo2d::circle(radius = 3mm) - std::geo2d::rect(width = 3mm, height = 2mm);
```

## Alternative difference operator

[![test](.test/difference_alt_operator.png)](.test/difference_alt_operator.md)

```µcad,difference_alt_operator
use std::*;

ops::difference() {
    geo2d::circle(radius = 3mm);
    geo2d::rect(width = 3mm, height = 2mm);
};
```
