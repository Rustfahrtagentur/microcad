# Test [`attributes_color`](/doc/attributes.md#L41)

## Code

```µcad
#[color = "#FFFFFF"]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq([c#color, (r = 1.0, g = 1.0, b = 1.0, a = 1.0)]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/attributes_color.png)
