# Conditionals

Conditions lead to different executions paths for different cases.

## If Statement

[![test](.test/if.png)](.test/if.log)

```µcad,if
fn f(x: Integer) {
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
