# Test [`types_quantity_weight`](/doc/types/quantity.md#L158)

## Code

```µcad
gram = 1000.0g;
kilogram = 1.0kg;
pound = 2.204623lb;

std::debug::assert([gram, kilogram].all_equal());

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_weight.png)
