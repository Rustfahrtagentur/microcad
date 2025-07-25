# Test [`function_module`](/doc/tests/statement_usage.md#L682)

## Code

```µcad
fn f() {
  mod m {}
} f();

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

![FAILED AS EXPECTED](/doc/tests/.test/function_module.png)
