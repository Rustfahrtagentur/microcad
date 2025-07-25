# Test [`code_post_init`](/doc/structure/workbench.md#L237)

## Code

```µcad
sketch wheel(radius: Length) {
    // initializer
    init( diameter: Length ) { radius = diameter / 2; }

    // building code starts here
    std::geo2d::circle(radius);
}

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/code_post_init.png)
