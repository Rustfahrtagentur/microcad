# Test [`module_expression_model`](/doc/tests/statement_usage.md#L208)

## Code

```µcad
mod k {
  __builtin::geo2d::circle(radius=1);
}

```

## Output

```,plain
```

## Errors

```,plain
error: Expression statement not available here
  ---> <from_str>:2:3
     |
   2 |   __builtin::geo2d::circle(radius=1);
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/module_expression_model.png)
