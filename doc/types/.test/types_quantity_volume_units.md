# Test [`types_quantity_volume_units`](/doc/types/quantity.md#L132)

## Code

```µcad
cubic_millimeter = 1000000.0mm³;
cubic_centimeter = 100.0cl;
cubic_meter = 0.001m³;
cubic_inch = 61.0237in³;
liter = 1.0l;
centiliter = 100.0cl;
milliliter = 1000.0ml;

std::debug::assert(cubic_millimeter == 1.0l);
std::debug::assert(cubic_centimeter == 1.0l);
std::debug::assert(cubic_meter == 1.0l);
std::debug::assert(centiliter == 1.0l);
std::debug::assert(milliliter == 1.0l);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_volume_units.png)
