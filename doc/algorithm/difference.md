# Difference

## Difference operator

In the following examples the difference of two circles is calculated.

```µCAD,difference.operator
circle(r = 3.0mm) - rect(3.0mm);
```

## Difference module

```µCAD,difference.module
difference() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```
