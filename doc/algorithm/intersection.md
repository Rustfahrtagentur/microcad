# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

```µCAD,intersection.operator
circle(r = 3.0mm) - rect(size = 3.0mm);
```

## Intersection module

```µCAD,intersection.module
intersection() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```
