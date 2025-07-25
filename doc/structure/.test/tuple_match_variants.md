# Test [`tuple_match_variants`](/doc/structure/arguments.md#L80)

## Output

```,plain
```

## Errors

```,plain
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x8db139f92f453785, Identifier: "y", Refer: 1:18 (17..18) in 0x8db139f92f453785, Identifier: "z", Refer: 1:29 (28..29) in 0x8db139f92f453785]
  ---> <from_str>:7:4
     |
   7 | f( (x=1cm, y=2cm, z=3cm) );
     |    ^^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x8db139f92f453785, Identifier: "y", Refer: 1:18 (17..18) in 0x8db139f92f453785, Identifier: "z", Refer: 1:29 (28..29) in 0x8db139f92f453785]
  ---> <from_str>:11:4
     |
  11 | f( p );
     |    ^^
     |
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x8db139f92f453785, Identifier: "y", Refer: 1:18 (17..18) in 0x8db139f92f453785]
  ---> <from_str>:14:4
     |
  14 | f( (x=1cm, y=2cm), z=3cm );
     |    ^^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x8db139f92f453785, Identifier: "z", Refer: 1:29 (28..29) in 0x8db139f92f453785]
  ---> <from_str>:15:4
     |
  15 | f( y=2cm, (x=1cm, z=3cm) );
     |    ^^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "x", Refer: 1:7 (6..7) in 0x8db139f92f453785, Identifier: "y", Refer: 1:18 (17..18) in 0x8db139f92f453785]
  ---> <from_str>:19:4
     |
  19 | f( q, z=3cm );
     |    ^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/tuple_match_variants.png)
