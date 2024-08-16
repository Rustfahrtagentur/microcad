# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

```µCAD,difference.operator
circle(r = 3.0mm) - rect(3.0mm);
```

## Difference module

```µCAD,difference.module
use * from std;

algorithm::difference() {
    geo2d::circle(radius = 3.0mm);
    geo2d::rect(width = 3.0mm, height = 2.0mm);
};
```
