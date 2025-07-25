# Test [`source_return`](/doc/tests/statement_usage.md#L54)

## Code

```µcad
return 1;

```

## Output

```,plain
```

## Errors

```,plain
error: Return statement not available here
  ---> <from_str>:1:1
     |
   1 | return 1;
     | ^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/source_return.png)
