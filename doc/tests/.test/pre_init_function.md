# Test [`pre_init_function`](/doc/tests/statement_usage.md#L234)

## Code

```µcad
sketch k() { 
  fn f() {} f();
init(l:Length) {} } k();

```

## Parse Error

```,plain
Statement not allowed prior initializers```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/pre_init_function.png)
