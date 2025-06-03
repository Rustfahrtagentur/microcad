# Part Properties

A property is defined by simply assigning an expression to an identifier.

[![test](.test/property_torus.png)](.test/property_torus.log)

```µcad,property_torus#todo
part torus(radius: length) {
    use std::geo2d::circle;

    // calculate inner from outer radius into property `inner`
    inner = radius / 2;

    // generate torus (and use field inner)
    circle(radius) - circle(inner);
}

// evaluate torus to get access to inner
t = torus(1cm);

// extract and display inner from generated part t
info("{t.inner}");
```

## Failures

[![test](.test/property_torus_fail.png)](.test/property_torus_fail.log)

```µcad,property_torus_fail#fail
part torus(radius) {} // Missing radius' type
```
