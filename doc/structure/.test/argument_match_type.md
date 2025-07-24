# Test [`argument_match_type`](../doc/structure/arguments.md#L36)

## Output

```,plain
```

## Errors

```,plain
error: Missing arguments: [Identifier: "c", Refer: 1:29 (28..29) in 0x78564033032ddc09]
  ---> <from_str>:3:3
     |
   3 | f(1.0, 2cm, 3cm²);
     |   ^^^^^^^^^^^^^^^
     |
```

## Test Result

![TODO](../doc/structure/.test/argument_match_type.png)
