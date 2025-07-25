# Test [`init_if`](/doc/tests/statement_usage.md#L388)

## Output

```,plain
```

## Errors

```,plain
error: If statement not available here
  ---> <from_str>:2:3
     |
   2 |   if std::math::PI == 3 { }
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Not implemented: evaluate if statement in function
  ---> <from_str>:2:3
     |
   2 |   if std::math::PI == 3 { }
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/tests/.test/init_if.png)
