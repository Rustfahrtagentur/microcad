# Test [`array_unit_bundling`](/doc/types/arrays.md#L11)

## Code

```µcad
// without bundling
l1 = [1mm, 2mm, 3mm];

// with bundling
l2 = [1, 2, 3]mm;

// are the same
std::debug::assert(l1 == l2);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/array_unit_bundling.png)
