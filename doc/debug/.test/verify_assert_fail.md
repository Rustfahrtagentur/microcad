# Test [`verify_assert_fail`](/doc/debug/README.md#L21)

## Code

```µcad
std::debug::assert(false, "this assertion fails");

```

## Output

```,plain
```

## Errors

```,plain
error: Assertion failed: false: this assertion fails
  ---> <from_str>:1:20
     |
   1 | std::debug::assert(false, "this assertion fails");
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/debug/.test/verify_assert_fail.png)
