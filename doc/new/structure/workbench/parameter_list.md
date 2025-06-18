# Parameter List

A 2D donut as circle with a hole.

[![test](.test/parameter_list.png)](.test/parameter_list.log)

```µcad,parameter_list#todo
// declare two parameters
part donut(outer: length, inner: length) {
    // parameters can be used anywhere within the part
    std::geo2d::circle(outer) - std::geo2d::circle(inner);
}

// generate donut of specific size
donut(2cm,1cm);
```
