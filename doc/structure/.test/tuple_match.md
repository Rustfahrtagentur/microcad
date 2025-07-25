# Test [`tuple_match`](/doc/structure/arguments.md#L62)

## Code

```µcad
// Function with three parameters: x, y, and z
fn f( x: Length, y: Length, z: Length ) {}

// Since we do not want to change x and y in the following statements,
// we prepare a tuple named plane:
plane = (x=1cm, y=2cm);

// Then we pass plane to f() three times with different z values
f( plane, z=3cm);
f( plane, z=6cm);
f( plane, z=9cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Missing arguments: [Identifier: "x", Refer: 2:7 (53..54) in 0x8c3d267be16f52ef, Identifier: "y", Refer: 2:18 (64..65) in 0x8c3d267be16f52ef]
  ---> <from_str>:9:4
     |
   9 | f( plane, z=3cm);
     |    ^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "x", Refer: 2:7 (53..54) in 0x8c3d267be16f52ef, Identifier: "y", Refer: 2:18 (64..65) in 0x8c3d267be16f52ef]
  ---> <from_str>:10:4
     |
  10 | f( plane, z=6cm);
     |    ^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "x", Refer: 2:7 (53..54) in 0x8c3d267be16f52ef, Identifier: "y", Refer: 2:18 (64..65) in 0x8c3d267be16f52ef]
  ---> <from_str>:11:4
     |
  11 | f( plane, z=9cm);
     |    ^^^^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/tuple_match.png)
