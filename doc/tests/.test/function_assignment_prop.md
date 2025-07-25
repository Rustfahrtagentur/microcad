# Test [`function_assignment_prop`](/doc/tests/statement_usage.md#L762)

## Code

```µcad
fn f() {
  prop a = 1;
} f();

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

![FAILED AS EXPECTED](/doc/tests/.test/function_assignment_prop.png)
