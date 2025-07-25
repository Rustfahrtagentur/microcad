# Test [`workbench_pub`](/doc/structure/functions.md#L120)

## Code

```µcad
part punched_disk(radius: Length) {
    pub fn inner() { radius/2 }   // error: cant use pub inside workbench
}

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK BUT IS TODO](/doc/structure/.test/workbench_pub.png)
