# Test [`workbench_fn_prop`](/doc/structure/functions.md#L130)

## Code

```µcad
part punched_disk(radius: Length) {
    fn inner() { 
        prop hole = radius/2;  // eval error: prop not allowed in function
    }
}

punched_disk(1cm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK BUT SHOULD FAIL](/doc/structure/.test/workbench_fn_prop.png)
