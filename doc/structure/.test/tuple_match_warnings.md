# Test [`tuple_match_warnings`](/doc/structure/arguments.md#L119)

## Output

```,plain
```

## Errors

```,plain
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x2b1194551c0fbef1, Identifier: "y", Refer: 1:18 (17..18) in 0x2b1194551c0fbef1]
  ---> <from_str>:2:4
     |
   2 | f( (x=1cm, y=2cm, v=5cm), z=3cm);  // warning: v is redundant
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/tuple_match_warnings.png)
