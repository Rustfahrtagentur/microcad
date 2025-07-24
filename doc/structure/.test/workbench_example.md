# Test [`workbench_example`](../doc/structure/functions.md#L98)

## Output

```,plain
```

## Errors

```,plain
error: Symbol inner not found.
  ---> <from_str>:8:29
     |
   8 |     circle(radius) - circle(inner());
     |                             ^^^^^^^
     |
error: Workbench circle cannot find initialization for those arguments
  ---> <from_str>:8:29
     |
   8 |     circle(radius) - circle(inner());
     |                             ^^^^^^^
     |
```

## Test Result

![FAIL](../doc/structure/.test/workbench_example.png)
