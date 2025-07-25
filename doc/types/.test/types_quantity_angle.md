# Test [`types_quantity_angle`](/doc/types/quantity.md#L74)

## Code

```µcad
pi = std::math::PI;
radian = 1rad * pi;
degree = 180°;
degree_ = 180deg;
grad = 200grad;
turn = 0.5turn;

std::debug::assert( [degree, degree_, grad, turn, radian].all_equal() );

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/types_quantity_angle.png)
