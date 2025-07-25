# Test [`types_quantity_length`](/doc/types/quantity.md#L59)

## Code

```µcad
millimeters = 1000mm;
centimeters = 100cm;
meters = 1m;
inches = 39.37007874015748in;

std::debug::assert( [millimeters, centimeters, meters, inches].all_equal() );

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_length.png)
