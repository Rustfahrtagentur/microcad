# Test [`function_assignment_const`](/doc/tests/statement_usage.md#L746)

## Code

```µcad
fn f() {
  const B = 1;
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
   2 |   const B = 1;
     |   ^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/function_assignment_const.png)
