# Test [`attributes_export_example`](/doc/attributes.md#L105)

## Code

```µcad
#[export("circle.svg")]
#[svg(style = "fill: skyblue; stroke: cadetblue; stroke-width: 2;")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq([c#export.filename, "circle.svg"]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/attributes_export_example.png)
