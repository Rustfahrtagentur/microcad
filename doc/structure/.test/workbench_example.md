# Test [`workbench_example`](/doc/structure/functions.md#L98)

## Code

```µcad
part punched_disk(radius: Length) {
    use std::geo2d::circle;

    // calculate inner from radius in a method
    fn inner() { radius/2 }

    // generate donut (and call inner)
    circle(radius) - circle(inner());
}

punched_disk(radius = 1cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Symbol inner not found.
  ---> <from_str>:8:29
     |
   8 |     circle(radius) - circle(inner());
     |                             ^^^^^^^
     |
error: Workbench circle cannot find initialization for those arguments
  ---> <from_str>:8:29
     |
   8 |     circle(radius) - circle(inner());
     |                             ^^^^^^^
     |
```

## Test Result

![FAIL](/doc/structure/.test/workbench_example.png)
