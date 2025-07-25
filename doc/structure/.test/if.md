# Test [`if`](/doc/structure/conditionals.md#L9)

## Code

```µcad
fn f( x: Scalar ) {
    if x == 5 or x == 4 {
        std::print("match");
    } else if x > 0 and x < 4 {
        std::print("no match");
    } else {
        std::print("invalid");
    }
}

f(5);  // prints "match"
f(1);  // prints "no match"
f(-1); // prints "invalid"
f(6);  // prints "invalid"

```

## Parse Error

```,plain
Parser error:  --> 4:12
  |
4 |     } else if x > 0 and x < 4 {
  |            ^---
  |
  = expected COMMENT or body```

Parser error:  --> 4:12
  |
4 |     } else if x > 0 and x < 4 {
  |            ^---
  |
  = expected COMMENT or body
## Test Result

![FAIL (TODO)](/doc/structure/.test/if.png)
