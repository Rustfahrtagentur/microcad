# Test [`workbench_module`](/doc/tests/statement_usage.md#L454)

## Code

```µcad
sketch k() {
  mod m {}
} k();

```

## Output

```,plain
```

## Errors

```,plain
error: Module statement not available here
  ---> <from_str>:2:3
     |
   2 |   mod m {}
     |   ^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/workbench_module.png)
