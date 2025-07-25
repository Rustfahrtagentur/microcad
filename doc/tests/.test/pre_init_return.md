# Test [`pre_init_return`](/doc/tests/statement_usage.md#L266)

## Code

```µcad
sketch k() { 
  return 1;
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed within workbenches```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_return.png)
