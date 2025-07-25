# Test [`function_function`](/doc/tests/statement_usage.md#L690)

## Code

```µcad
fn f() {
  fn f() {}
} f();

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

![FAILED AS EXPECTED](/doc/tests/.test/function_function.png)
