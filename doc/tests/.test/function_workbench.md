# Test [`function_workbench`](/doc/tests/statement_usage.md#L674)

## Code

```µcad
fn f() {
  sketch s() {}
} f();

```

## Output

```,plain
```

## Errors

```,plain
error: sketch statement not available here
  ---> <from_str>:2:3
     |
   2 |   sketch s() {}
     |   ^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/function_workbench.png)
