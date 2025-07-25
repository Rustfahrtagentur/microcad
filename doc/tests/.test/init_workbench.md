# Test [`init_workbench`](/doc/tests/statement_usage.md#L332)

## Code

```µcad
sketch k() { init(l:Length) {
  sketch f() {}
} } k(1cm);

```

## Output

```,plain
```

## Errors

```,plain
error: sketch statement not available here
  ---> <from_str>:2:3
     |
   2 |   sketch f() {}
     |   ^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_workbench.png)
