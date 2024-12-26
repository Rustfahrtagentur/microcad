# Module Functions

Modules can have functions which may calculate and return values, generate geometries, set module members or use other functions.

Functions consist of statements.

Example which can generate a 2D donut of a given radius:

[![test](.test/functions_donut.png)](.test/functions_donut.log)

```µcad,functions_donut
module donut(radius: Length) {
    use std::geo2d::circle;

    // calculate inner from radius in a method
    function inner() { radius/2 }

    // generate donut
    circle(radius) - circle(inner());
}

donut(radius = 1cm);
```

[![test](.test/functions_donut_pub.png)](.test/functions_donut_pub.log)

```µcad,functions_donut_pub
module donut(radius: Length) {
    // calculate inner from radius in a method
    function inner() { radius/2 }
}

d = donut(radius = 1cm).inner();
std::print("{d}");
```
