# Test [`verify_error`](../doc/debug/README.md#L29)

## Output

```,plain
```

## Errors

```,plain
error: Builtin error: this should not have happened
  ---> <from_str>:1:12
     |
   1 | std::error("this should not have happened");
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](../doc/debug/.test/verify_error.png)
