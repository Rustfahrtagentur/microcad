# Test [`module_pub_use`](../doc/tests/statement_usage.md#L144)

## Output

```,plain
```

## Errors

```,plain
error: Unexpected stack frame of type 'module' cannot store std
  ---> <from_str>:2:11
     |
   2 |   pub use std;
     |           ^^^
     |
```

## Test Result

![TODO](../doc/tests/.test/module_pub_use.png)
