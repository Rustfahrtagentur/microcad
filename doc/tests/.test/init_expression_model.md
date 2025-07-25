# Test [`init_expression_model`](/doc/tests/statement_usage.md#L436)

## Code

```µcad
sketch k() { init(l:Length) {
  __builtin::geo2d::circle(radius=1);
} } k(1cm);

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

![FAILED AS EXPECTED](/doc/tests/.test/init_expression_model.png)
