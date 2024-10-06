# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

```µCAD,intersection.operator
std::geo2d::circle(radius = 3.0) & std::geo2d::rect(x=0.0, y=0.0, width = 3.0, height = 2.0);
```

## Intersection module

```µCAD,intersection.module
std::algorithm::intersection() {
    std::geo2d::circle(radius = 3.0);
    std::geo2d::rect(x=0.0, y=0.0, width = 3.0, height = 2.0);
}
```
