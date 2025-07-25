# Test [`source_assignment_prop`](/doc/tests/statement_usage.md#L84)

## Code

```µcad
prop a = 1;

```

## Output

```,plain
```

## Errors

```,plain
error: Assignment statement not available here
  ---> <from_str>:1:1
     |
   1 | prop a = 1;
     | ^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/source_assignment_prop.png)
