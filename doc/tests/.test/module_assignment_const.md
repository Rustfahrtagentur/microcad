# Test [`module_assignment_const`](/doc/tests/statement_usage.md#L176)

## Code

```µcad
mod k {
  const B = 1;
}

```

## Output

```,plain
```

## Errors

```,plain
error: No variables allowed in modules
  ---> <from_str>:2:3
     |
   2 |   const B = 1;
     |   ^^^^^^^^^^^^
     |
```

## Test Result

![FAIL](/doc/tests/.test/module_assignment_const.png)
