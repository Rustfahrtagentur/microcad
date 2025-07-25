# Test [`pre_init_if`](/doc/tests/statement_usage.md#L274)

## Code

```µcad
sketch k() { 
  if std::math::PI == 3 { }
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_if.png)
