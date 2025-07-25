# Test [`init_assignment_prop`](/doc/tests/statement_usage.md#L420)

## Code

```µcad
sketch k() { init(l:Length) {
  prop a = 1;
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
   2 |   prop a = 1;
     |   ^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_assignment_prop.png)
