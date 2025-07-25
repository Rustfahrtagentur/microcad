# Test [`property`](/doc/structure/workbench.md#L272)

## Output

```,plain
```

## Errors

```,plain
error: Workbench wheel cannot find initialization for those arguments
  ---> <from_str>:15:11
     |
  15 | t = wheel(1cm);
     |           ^^^
     |
error: Symbol info not found.
  ---> <from_str>:18:1
     |
  18 | info("outer: {t.outer}");
     | ^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Symbol info not found.
  ---> <from_str>:19:1
     |
  19 | info("inner: {t.inner}");
     | ^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/property.png)
