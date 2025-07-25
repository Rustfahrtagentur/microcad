# Test [`first_example`](/README.md#L82)

## Code

```µcad
// We have part called `lid` with three parameters
part lid(
    thickness = 1.6mm,
    inner_diameter = 16cm,
    height = 20mm,
) {
    // Calculate the outer diameter
    outer_diameter = 2 * thickness + inner_diameter;

    // Create two cylinders, one for the outer and one for the inner
    outer = std::geo3d::cylinder(d = outer_diameter, h = height);
    inner = std::ops::translate(z = thickness) std::geo3d::cylinder(d = inner_diameter, h = height);

    // Calculate the difference between two translated cylinders and output them
    outer - inner;
}

// `l` is the instance of the lid model
lid();

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/.test/first_example.png)
