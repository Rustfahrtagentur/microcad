# Module Fields

A field is defined by simply assigning an expression to an identifier.

```µCAD,torus
// module `torus`
module torus(radius: length) {
    use circle from std::geo2d;

    // calculate inner from radius into field `inner`
    inner = radius/2;

    // generate torus (and use field `inner`)
    circle(radius) - circle(inner);
}

// evaluate `torus` to get access to `inner`
t = torus(1cm);

// extract and display `inner` from generated module `t`
info("{t.inner}");
```

## Failures

```µCAD,fail.torus#fail
module torus(radius) {} // Missing radius' type
```
