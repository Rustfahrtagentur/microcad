# Test [`property_wrong`](/doc/structure/workbench.md#L298)

## Output

```,plain
```

## Errors

```,plain
info: outer: 10mm
  ---> <from_str>:14:11
     |
  14 | std::info("outer: {t.outer}");
     |           ^^^^^^^^^^^^^^^^^^
     |
error: Property not found: inner
  ---> <from_str>:16:20
     |
  16 | std::info("inner: {t.inner}");
     |                    ^^^^^^^
     |
info: inner: <invalid>
  ---> <from_str>:16:11
     |
  16 | std::info("inner: {t.inner}");
     |           ^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/property_wrong.png)
