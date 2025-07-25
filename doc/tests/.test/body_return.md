# Test [`body_return`](/doc/tests/statement_usage.md#L608)

## Code

```µcad
{
  return 1;
}

```

## Output

```,plain
```

## Errors

```,plain
error: Return statement not available here
  ---> <from_str>:2:3
     |
   2 |   return 1;
     |   ^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/body_return.png)
