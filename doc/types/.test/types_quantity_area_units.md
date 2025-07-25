# Test [`types_quantity_area_units`](/doc/types/quantity.md#L102)

## Code

```µcad
square_millimeter = 100000mm²;
square_centimeter = 1000cm²;
square_meter = 0.1m²;
square_inch = 155in²;

std::debug::assert(square_millimeter == 0.1m²);
std::debug::assert(square_centimeter == 0.1m²);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_area_units.png)
