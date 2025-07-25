# Test [`export_attributes`](/doc/export.md#L42)

## Code

```µcad
#[export("rect.svg")] // Will be exported to `rect.svg`
std::geo2d::rect(42mm);

#[export("circle.svg")]  // Will be exported to `circle.svg`
std::geo2d::circle(r = 42mm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/export_attributes.png)
