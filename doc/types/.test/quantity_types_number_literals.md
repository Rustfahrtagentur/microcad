# Test [`quantity_types_number_literals`](/doc/types/quantity.md#L27)

## Code

```µcad
// declare variable `height` of type `Length` to 1.4 Meters
height = 1.4m;

// use as *default* value in parameter list
fn f( height = 1m ) {}

// calculate a `Length` called `width` by multiplying the
// `height` with `Scalar` `2` and add ten centimeters
width = height * 2 + 10cm;

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/types/.test/quantity_types_number_literals.png)
