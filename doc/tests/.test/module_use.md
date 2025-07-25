# Test [`module_use`](/doc/tests/statement_usage.md#L136)

## Code

```µcad
mod k {
  use std;
}

```

## Output

```,plain
```

## Errors

```,plain
error: Unexpected stack frame of type 'module' cannot store std
  ---> <from_str>:2:7
     |
   2 |   use std;
     |       ^^^
     |
```

## Test Result

![FAIL](/doc/tests/.test/module_use.png)
