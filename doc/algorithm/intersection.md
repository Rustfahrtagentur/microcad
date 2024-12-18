# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

```µcad,intersection.operator
std::geo2d::circle(radius = 3.0mm) & std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Intersection module

```µcad,intersection.module
std::algorithm::intersection() {
    std::geo2d::circle(radius = 3.0mm);
    std::geo2d::rect(width = 3.0mm, height = 2.0mm);
}
```
