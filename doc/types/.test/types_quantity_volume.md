# Test [`types_quantity_volume`](/doc/types/quantity.md#L118)

## Code

```µcad
a = 3mm;
b = 2mm;
c = 4mm;

volume = a * b * c;

std::debug::assert(volume == 24mm³);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_volume.png)
