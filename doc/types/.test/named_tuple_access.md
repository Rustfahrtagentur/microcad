# Test [`named_tuple_access`](/doc/types/tuples.md#L10)

## Code

```µcad
use std::debug::assert_eq;

tuple = (width=10cm, depth=10cm, volume=1l);

assert_eq([tuple.width, 10cm]);
assert_eq([tuple.depth, 10cm]);
assert_eq([tuple.volume, 1l]);

assert_eq([tuple, (width=10cm, depth=10cm, volume=1l)]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/named_tuple_access.png)
