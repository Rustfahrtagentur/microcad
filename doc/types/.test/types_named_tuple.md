# Test [`types_named_tuple`](/doc/types/named_tuple.md#L9)

## Code

```µcad
a = (width=10cm, depth=10cm, volume=1l);

std::debug::assert(a.width == 10cm);
std::debug::assert(a.depth == 10cm);
std::debug::assert(a.volume == 1l);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_named_tuple.png)
