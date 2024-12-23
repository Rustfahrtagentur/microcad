# Module Fields

A field is defined by simply assigning an expression to an identifier.

![test](.test/fields_torus.png)

```µcad,fields_torus#todo
// module torus
module torus(radius: length) {
    use std::geo2d::circle;

    // calculate inner from radius into field inner
    inner = radius / 2.0;

    // generate torus (and use field inner)
    circle(radius) - circle(inner);
}

// evaluate torus to get access to inner
t = torus(1cm);

// extract and display inner from generated module t
info("{t.inner}");
```

## Failures

![test](.test/fields_torus_fail.png)

```µcad,fields_torus_fail#fail
module torus(radius) {} // Missing radius' type
```
