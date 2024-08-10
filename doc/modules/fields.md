# Module Fields

```µCAD,donut
module donut(radius) {
    use circle from std::geo2d;

    // calculate inner from radius
    inner = radius/2;

    // generate donut
    circle(radius) - circle(inner);
}
```
