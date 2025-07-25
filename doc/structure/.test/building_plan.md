# Test [`building_plan`](/doc/structure/workbench.md#L95)

## Code

```µcad
// sketch with a `radius` as building plan
sketch wheel(radius: Length) {
    // access property `radius` from the building plan
    std::geo2d::circle(radius);
}

std::debug::assert_eq([wheel(5cm).radius, 5cm]);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/building_plan.png)
