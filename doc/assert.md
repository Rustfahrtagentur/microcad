# Assertions

Assertions define constrains of parameters or cases.

one form of assertion is a function which gets an expression.
If the expression computes to `false` a compile error will occur at
that point.

```µCAD,assert
use * from std;
assert(true);
```

```µCAD,assert_fail#fail
use * from std;
assert(false);
```
