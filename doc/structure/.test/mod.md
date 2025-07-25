# Test [`mod`](/doc/structure/functions.md#L66)

## Code

```µcad
// module math
mod math {
    // pow cannot be called from outside math
    fn pow( x: Scalar, n: Integer ) {
        if n == 1 {
            x   // return x
        } else {
            x * pow(x, n-1) // return recursive product
        }
    }

    // square is callable from outside math
    pub fn square(x: Scalar) {
        // call internal pow
        pow(x, 2.0)
    }
}

// call square in math
math::square(2.0);
math::pow(2.0, 5);  // error: pow is private

```

## Output

```,plain
```

## Errors

```,plain
error: Symbol pow not found.
  ---> <from_str>:15:9
     |
  15 |         pow(x, 2.0)
     |         ^^^^^^^^^^^
     |
error: Not implemented: evaluate if statement in function
  ---> <from_str>:5:9
     |
   5 |         if n == 1 {
     |         ^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/mod.png)
