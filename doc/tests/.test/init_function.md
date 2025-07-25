# Test [`init_function`](/doc/tests/statement_usage.md#L348)

## Code

```µcad
sketch k() { init(l:Length) {
  fn f() {}
} } k(1cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Function statement not available here
  ---> <from_str>:2:3
     |
   2 |   fn f() {}
     |   ^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_function.png)
