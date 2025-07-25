# Test [`pre_init_workbench`](/doc/tests/statement_usage.md#L218)

## Code

```µcad
sketch k() { 
  sketch f() {} f();
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed within workbenches```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_workbench.png)
