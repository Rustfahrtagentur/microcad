# Test [`return_twice`](/doc/structure/functions.md#L49)

## Code

```µcad
fn pow( x: Scalar, n: Integer ) {
    if n == 1 {
        x 
    }
    x * pow(n-1)  // error: unexpected code
}

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/return_twice.png)
