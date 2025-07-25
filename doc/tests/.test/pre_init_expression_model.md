# Test [`pre_init_expression_model`](/doc/tests/statement_usage.md#L322)

## Code

```µcad
sketch k() { 
  __builtin::geo2d::circle(radius=1);
init(l:Length) {} }

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_expression_model.png)
