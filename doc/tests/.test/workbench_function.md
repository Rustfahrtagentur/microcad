# Test [`workbench_function`](/doc/tests/statement_usage.md#L462)

## Code

```µcad
sketch k() {
  fn f() {} f();
} k();

```

## Output

```,plain
```

## Errors

```,plain
error: Symbol f not found.
  ---> <from_str>:2:13
     |
   2 |   fn f() {} f();
     |             ^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/workbench_function.png)
