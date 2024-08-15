# Module Fields

```µCAD,donut
module donut(radius: length) {
    use circle from std::geo2d;

    // calculate inner from radius
    inner = radius/2;

    // generate donut
    circle(radius) - circle(inner);
}
```

## Failures

```µCAD,fail.donut#fail
module donut(radius) {} // Missing type parameter
```
