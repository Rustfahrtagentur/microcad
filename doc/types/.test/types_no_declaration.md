# Test [`types_no_declaration`](/doc/types/README.md#L38)

## Parse Error

```,plain
Parser error:  --> 1:10
  |
1 | x: Length;         // error
  |          ^---
  |
  = expected COMMENT```

## Test Result

![FAILED AS EXPECTED](/doc/types/.test/types_no_declaration.png)
