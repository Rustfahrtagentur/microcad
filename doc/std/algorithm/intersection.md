# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

![test](.test/intersection_operator.png)
[see build log](.test/intersection_operator.log)

```µcad,intersection_operator
std::geo2d::circle(radius = 3.0mm) & std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Intersection module

![test](.test/intersection_module.png)
[see build log](.test/intersection_module.log)

```µcad,intersection_module
std::algorithm::intersection() {
    std::geo2d::circle(radius = 3.0mm);
    std::geo2d::rect(width = 3.0mm, height = 2.0mm);
}
```
