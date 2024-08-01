# Intersection

## Intersection operator

In the following examples the intersection of two circles is calculated.

```µCAD
circle(r = 3.0mm) - rect(3.0mm);
```

## Intersection module

```µCAD
intersection() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```
