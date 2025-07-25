# Test [`op_example`](/doc/structure/op.md#L12)

## Code

```µcad
// define operation nop without parameters
op nop() { @children }

// use operation nop on a circle
nop() std::geo2d::circle(radius = 1cm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/op_example.png)
