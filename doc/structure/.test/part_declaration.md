# Test [`part_declaration`](../doc/structure/workbench.md#L43)

## Output

```,plain
```

## Errors

```,plain
error: Symbol FACTOR not found.
  ---> <from_str>:8:5
     |
   8 |     init(diameter: Length) {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Symbol into_diameter not found.
  ---> <from_str>:21:21
     |
  21 |     prop diameter = into_diameter(radius);
     |                     ^^^^^^^^^^^^^^^^^^^^^
     |
error: Workbench circle cannot find initialization for those arguments
error: Symbol assert_eq not found.
  ---> <from_str>:34:1
     |
  34 | assert_eq([d.radius, 1cm]);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Symbol into_diameter not found.
  ---> <from_str>:21:21
     |
  21 |     prop diameter = into_diameter(radius);
     |                     ^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "v", Refer: <no_ref>]
```

## Test Result

![TODO](../doc/structure/.test/part_declaration.png)
