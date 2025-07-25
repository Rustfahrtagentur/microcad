# Test [`pre_init_marker`](/doc/tests/statement_usage.md#L282)

## Code

```µcad
sketch k() { 
  @children
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_marker.png)
