# Test [`measure`](/doc/nodes/measures.md#L9)

## Code

```µcad
__builtin::assert_eq([
    // use measure area() on a circle
    std::geo2d::circle(radius=10mm).area(),

    // circle area formula for comparison
    10mm * 10mm * std::math::PI
]);

```

