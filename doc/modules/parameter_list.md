# Module Parameter List

A 2D donut as circle with a hole.

```µCAD,parameters
// declare two parameters
module donut(outer: length, inner: length) {
    // parameters can be used anywhere within the module
    std::geo2d::circle(outer) - std::geo2d::circle(inner);
}

// generate donut of specific size
donut(2cm,1cm);
```