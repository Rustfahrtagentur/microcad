# Test [`body_module`](/doc/tests/statement_usage.md#L568)

## Code

```µcad
{
  mod m {}
}

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

![FAILED AS EXPECTED](/doc/tests/.test/body_module.png)
