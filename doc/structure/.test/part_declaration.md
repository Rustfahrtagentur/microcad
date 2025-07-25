# Test [`part_declaration`](/doc/structure/workbench.md#L43)

## Code

```µcad
// sketch with a `radius` as building plan
sketch wheel(radius: Length) {

    // init code
    const FACTOR = 2;

    // initializer
    init(diameter: Length) {
        // set `radius`
        radius = diameter / FACTOR;
    }

    // function (sub routine)
    fn into_diameter(radius: Length) {
        return radius * FACTOR;
    }

    // building code begins

    // set a property which can be seen from outside
    prop diameter = into_diameter(radius);
    // local variable
    i = 1;
    
    // create circle
    std::geo2d::circle(radius);
}

use std::debug::assert;

// call sketch with diameter
d = wheel(diameter = 2cm)
// check radius
assert_eq([d.radius, 1cm]);

// call sketch with radius
r = wheel(radius = 1cm)
// check diameter
assert([r.diameter, 2cm]);

```

## Output

```,plain
```

## Errors

```,plain
error: Symbol FACTOR not found.
  ---> <from_str>:8:5
     |
   8 |     init(diameter: Length) {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Symbol into_diameter not found.
  ---> <from_str>:21:21
     |
  21 |     prop diameter = into_diameter(radius);
     |                     ^^^^^^^^^^^^^^^^^^^^^
     |
error: Workbench circle cannot find initialization for those arguments
error: Symbol assert_eq not found.
  ---> <from_str>:34:1
     |
  34 | assert_eq([d.radius, 1cm]);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: Symbol into_diameter not found.
  ---> <from_str>:21:21
     |
  21 |     prop diameter = into_diameter(radius);
     |                     ^^^^^^^^^^^^^^^^^^^^^
     |
error: Missing arguments: [Identifier: "v", Refer: <no_ref>]
```

## Test Result

![TODO](/doc/structure/.test/part_declaration.png)
