# Module Functions

```µCAD,donut
module donut(radius: length) {
    use circle from std::geo2d;

    // calculate inner from radius in a method
    function inner() { radius/2 }

    // generate donut
    circle(radius) - circle(inner());
}
```
