# Test [`function_if`](/doc/tests/statement_usage.md#L730)

## Code

```µcad
fn f() {
  if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
} f();

```

## Output

```,plain
```

## Errors

```,plain
error: Not implemented: evaluate if statement in function
  ---> <from_str>:2:3
     |
   2 |   if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAIL](/doc/tests/.test/function_if.png)
