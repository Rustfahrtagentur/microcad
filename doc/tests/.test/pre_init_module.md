# Test [`pre_init_module`](/doc/tests/statement_usage.md#L226)

## Code

```µcad
sketch k() { 
  mod m {}
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed within workbenches```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_module.png)
