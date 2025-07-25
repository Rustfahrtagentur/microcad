# Test [`return`](/doc/structure/functions.md#L35)

## Code

```µcad
fn pow( x: Scalar, n: Integer ) {
    if n == 1 {
        x   // return x
    } else {
        x * pow(n-1) // return recursive product
    }
}

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/return.png)
