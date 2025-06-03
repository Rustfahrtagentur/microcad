# Part Fields

A field is defined by simply assigning an expression to an identifier.

[![test](.test/fields_torus.png)](.test/fields_torus.log)

```µcad,fields_torus#todo
part torus(radius: length) {
    use std::geo2d::circle;

    // calculate inner from radius into field inner
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

[![test](.test/fields_torus_fail.png)](.test/fields_torus_fail.log)

```µcad,fields_torus_fail#fail
part torus(radius) {} // Missing radius' type
```
