# Test [`init_module`](/doc/tests/statement_usage.md#L340)

## Code

```µcad
sketch k() { init(l:Length) {
  mod m {}
} } k(1cm);

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

![FAILED AS EXPECTED](/doc/tests/.test/init_module.png)
