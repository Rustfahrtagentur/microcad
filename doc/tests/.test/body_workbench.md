# Test [`body_workbench`](/doc/tests/statement_usage.md#L560)

## Output

```,plain
```

## Errors

```,plain
error: sketch statement not available here
  ---> <from_str>:2:3
     |
   2 |   sketch f() {} f();
     |   ^^^^^^^^^^^^^
     |
error: Symbol f not found.
  ---> <from_str>:2:17
     |
   2 |   sketch f() {} f();
     |                 ^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/body_workbench.png)
