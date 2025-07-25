# Test [`types_primitive_string`](/doc/types/primitive_types.md#L29)

## Code

```µcad
text = "Hello µcad!";
std::debug::assert_eq([std::count(text), 11]);

// logging
std::print(text);

```

## Output

```,plain
Hello µcad!
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_primitive_string.png)
