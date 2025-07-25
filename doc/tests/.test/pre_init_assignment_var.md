# Test [`pre_init_assignment_var`](/doc/tests/statement_usage.md#L298)

## Code

```µcad
sketch k() { 
  a = 1;
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_assignment_var.png)
