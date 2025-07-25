# Test [`init_assignment_const`](/doc/tests/statement_usage.md#L404)

## Code

```µcad
sketch k() { init(l:Length) {
  const B = 1;
} } k(1cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Assignment statement not available here
  ---> <from_str>:2:3
     |
   2 |   const B = 1;
     |   ^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_assignment_const.png)
