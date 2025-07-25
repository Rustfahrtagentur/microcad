# Test [`init_expression`](/doc/tests/statement_usage.md#L428)

## Code

```µcad
sketch k() { init(l:Length) {
  1 + 2;
} } k(1cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Expression statement not available here
  ---> <from_str>:2:3
     |
   2 |   1 + 2;
     |   ^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_expression.png)
