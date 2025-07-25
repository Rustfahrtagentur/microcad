# Test [`multiplicity_tuple_array`](/doc/structure/arguments.md#L157)

## Code

```µcad
std::ops::translate([(x=-4mm, y=-4mm), (x=-4mm, y=4mm), (x=4mm, y=-4mm), (x=4mm, y=4mm)]) 
    std::geo2d::rect(width = 2mm, height = 2mm);

```

## Output

```,plain
```

## Errors

```,plain
error: Workbench translate cannot find initialization for those arguments
  ---> <from_str>:1:21
     |
   1 | std::ops::translate([(x=-4mm, y=-4mm), (x=-4mm, y=4mm), (x=4mm, y=-4mm), (x=4mm, y=4mm)]) 
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/multiplicity_tuple_array.png)
