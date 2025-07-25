# Test [`pre_init_code`](/doc/structure/workbench.md#L172)

## Code

```µcad
sketch wheel(radius: Length) {
    // init code
    const FACTOR = 2.0;

    // initializer
    init(diameter: Length) { into_radius(radius); }

    // function
    fn into_radius( diameter: Length ) {
        // use constant FACTOR from init code
        return diameter / FACTOR;
    }

    // set property diameter and use FACTOR from init code
    prop diameter = radius * FACTOR;
    
    // code body
    std::geo2d::circle(radius);
}

__builtin::assert(wheel(5cm).radius == 5cm);
__builtin::assert(wheel(5cm).diameter == 10cm);

```

## Output

```,plain
```

## Errors

```,plain
error: Property not found: diameter
  ---> <from_str>:22:19
     |
  22 | __builtin::assert(wheel(5cm).diameter == 10cm);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "v", Refer: <no_ref>]
  ---> <from_str>:22:19
     |
  22 | __builtin::assert(wheel(5cm).diameter == 10cm);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![TODO](/doc/structure/.test/pre_init_code.png)
