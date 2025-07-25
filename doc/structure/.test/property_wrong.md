# Test [`property_wrong`](/doc/structure/workbench.md#L298)

## Code

```µcad
sketch wheel(outer: length) {
    use std::geo2d::circle;

    // `inner` is declared as variable and may not be read
    // from outside this workbench
    inner = outer / 2;

    circle(outer) - circle(inner);
}

t = wheel(outer = 1cm);

// you can still extract and display `outer`
std::info("outer: {t.outer}");
// error: but you cannot access `inner` anymore
std::info("inner: {t.inner}");

```

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
