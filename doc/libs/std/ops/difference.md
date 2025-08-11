# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

[![test](.test/difference_operator.svg)](.test/difference_operator.log)

```µcad,difference_operator
std::geo2d::circle(radius = 10mm) - std::geo2d::rect(size = 2mm);
```

## Alternative difference operator

[![test](.test/difference_alt_operator.svg)](.test/difference_alt_operator.log)

```µcad,difference_alt_operator
use std::*;

{
    geo2d::circle(radius = 10mm);
    geo2d::rect(size = 2mm);
}.ops::difference();
```
