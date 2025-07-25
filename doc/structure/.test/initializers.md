# Test [`initializers`](/doc/structure/workbench.md#L112)

## Code

```µcad
part wheel(radius: Length) {
    init( radius: Length ) {} // error: same parameters as in building plan
}

wheel(radius = 1.0mm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK BUT IS TODO](/doc/structure/.test/initializers.png)
