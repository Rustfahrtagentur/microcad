# Test [`module_if`](/doc/tests/statement_usage.md#L160)

## Code

```µcad
mod k {
  if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
}

```

## Output

```,plain
```

## Errors

```,plain
error: If statement not available here
  ---> <from_str>:2:3
     |
   2 |   if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/module_if.png)
