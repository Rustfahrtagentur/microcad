# Test [`body_function`](/doc/tests/statement_usage.md#L576)

## Code

```µcad
{
  fn f() {} f();
}

```

## Output

```,plain
```

## Errors

```,plain
error: Function statement not available here
  ---> <from_str>:2:3
     |
   2 |   fn f() {} f();
     |   ^^^^^^^^^
     |
error: Symbol f not found.
  ---> <from_str>:2:13
     |
   2 |   fn f() {} f();
     |             ^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/body_function.png)
