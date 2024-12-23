# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

![test](.banner/intersection_operator.png)

```µcad,intersection_operator
std::geo2d::circle(radius = 3mm) & std::geo2d::rect(width = 3mm, height = 2mm);
```

## Intersection module

![test](.banner/intersection_module.png)

```µcad,intersection_module
std::algorithm::intersection() {
    std::geo2d::circle(radius = 3mm);
    std::geo2d::rect(width = 3mm, height = 2mm);
}
```
