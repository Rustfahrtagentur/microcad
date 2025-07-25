# Test [`module_assignment_prop`](/doc/tests/statement_usage.md#L192)

## Code

```µcad
mod k {
  prop a = 1;
}

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
error: No variables allowed in modules
  ---> <from_str>:2:3
     |
   2 |   prop a = 1;
     |   ^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/module_assignment_prop.png)
