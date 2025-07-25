# Test [`toml_import`](/doc/import.md#L26)

## Code

```µcad
data = std::import("example.toml");

std::debug::assert_eq([data.M10.diameter, 10.0]);
std::debug::assert_eq([data.M6.pitch, 1.0]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/.test/toml_import.png)
