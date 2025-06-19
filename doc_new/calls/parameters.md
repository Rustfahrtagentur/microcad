# Call Parameters

## Parameter Matching

TODO

## Parameter Multiplicity

Each parameter can also be given as a list with elements of the parameter's type.
Each list element will be evaluated. This is called *parameter multiplicity*.
This way, we can intuitively express a call that is called for each parameter variant.

The following example will produce 4 rectangles on different positions:

[![test](.test/parameter_multiplicity_example_A.png)](.test/parameter_multiplicity_example_A.log)

```µcad,parameter_multiplicity_example_A
std::translate(x = [-4mm, 4mm], y = [-4mm, 4mm]) 
    std::geo2d::rect(width = 2mm, height = 2mm);
```

The example results in the following calls:

[![test](.test/parameter_multiplicity_example_B.png)](.test/parameter_multiplicity_example_B.log)

```µcad,parameter_multiplicity_example_B
std::translate(x = -4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = -4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
```

Normally, this would require 2 nested *for loops* which are not available in *µcad*.
