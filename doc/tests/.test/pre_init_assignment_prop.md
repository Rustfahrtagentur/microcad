# Test [`pre_init_assignment_prop`](/doc/tests/statement_usage.md#L306)

## Code

```µcad
sketch k() { 
  prop a = 1;
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_assignment_prop.png)
