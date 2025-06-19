# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

[![test](.test/intersection_operator.png)](.test/intersection_operator.log)

```µcad,intersection_operator
std::geo2d::circle(radius = 3mm) & std::geo2d::rect(width = 3mm, height = 2mm);
```

## Alternative intersection operator

[![test](.test/intersection_alt_operator.png)](.test/intersection_alt_operator.log)

```µcad,intersection_alt_operator
std::algorithm::intersection() {
    std::geo2d::circle(radius = 3mm);
    std::geo2d::rect(width = 3mm, height = 2mm);
}
```
