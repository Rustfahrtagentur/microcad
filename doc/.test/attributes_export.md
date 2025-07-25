# Test [`attributes_export`](/doc/attributes.md#L74)

## Code

```µcad
#[export("circle.svg")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq([c#export.filename, "circle.svg"]);
std::debug::assert_eq([c#export.id, "svg"]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/attributes_export.png)
