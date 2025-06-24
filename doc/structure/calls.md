# Calls

## Calling Functions

TODO

## Calling Workbenches

TODO

## Call Arguments

### Argument Matching

TODO

### Argument Multiplicity

Each argument can also be given in a array of elements of a parameter's type.
Each list element will then be evaluated for each of the array's values.
This is called *argument multiplicity*.
This way, we can intuitively express a call that is called for each argument variant.

The following example will produce 4 rectangles on different positions:

[![test](.test/multiplicity_example_A.png)](.test/multiplicity_example_A.log)

```µcad,multiplicity_example_A
std::translate(x = [-4mm, 4mm], y = [-4mm, 4mm]) 
    std::geo2d::rect(width = 2mm, height = 2mm);
```

The example results in the following calls:

[![test](.test/multiplicity_example_B.png)](.test/multiplicity_example_B.log)

```µcad,multiplicity_example_B
std::translate(x = -4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = -4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
```

Normally, this would require 2 nested *for loops* which are not available in *µcad*.
