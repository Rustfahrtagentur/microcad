# Test [`missed_property`](/doc/structure/workbench.md#L152)

## Output

```,plain
```

## Errors

```,plain
error: Workbench plan incomplete. Missing properties: [Identifier: "radius", Refer: 1:14 (13..19) in 0xf3cdaa8ec03331ff]
  ---> <from_str>:2:5
     |
   2 |     init( width: Length ) { 
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/missed_property.png)
