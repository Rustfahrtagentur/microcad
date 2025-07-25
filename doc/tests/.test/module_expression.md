# Test [`module_expression`](/doc/tests/statement_usage.md#L200)

## Code

```µcad
mod k {
  1 + 2;
}

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

![FAILED AS EXPECTED](/doc/tests/.test/module_expression.png)
