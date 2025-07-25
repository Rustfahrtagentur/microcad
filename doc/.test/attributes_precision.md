# Test [`attributes_precision`](/doc/attributes.md#L56)

## Code

```µcad
#[resolution = 200%]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq([c#resolution, 200%]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/attributes_precision.png)
