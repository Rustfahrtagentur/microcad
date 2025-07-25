# Test [`illegal_workbench_statement`](/doc/structure/workbench.md#L253)

## Code

```µcad
sketch wheel(radius: Length) {
    sketch axis(length: Length) {}
}

wheel(radius = 1.0mm);

```

## Output

```,plain
```

## Errors

```,plain
error: sketch statement not available here
  ---> <from_str>:2:5
     |
   2 |     sketch axis(length: Length) {}
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/illegal_workbench_statement.png)
