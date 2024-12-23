# Parameter Multiplicity

Each parameter can also be given as a list with elements of the parameter's type.
Each list element will be evaluated. This is called *parameter multiplicity*.
This way, we can intuitively express a call that is called for each parameter variant.

The following example will produce 4 rectangles on different positions:

![test](.test/parameter_multiplicity_example_A.png)
[see build log](.test/parameter_multiplicity_example_A.log)

```µcad,parameter_multiplicity_example_A
std::translate(x = [-4mm, 4mm], y = [-4mm, 4mm]) 
    std::geo2d::rect(width = 2mm, height = 2mm);
```

The example results in the following calls:

![test](.test/parameter_multiplicity_example_B.png)
[see build log](.test/parameter_multiplicity_example_B.log)

```µcad,parameter_multiplicity_example_B
std::translate(x = -4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = -4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::translate(x = 4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
```

Normally, this would require 2 nested *for loops* which are not available in *µcad*.

![test](.test/parameter_multiplicity_example_D.png)
[see build log](.test/parameter_multiplicity_example_D.log)

```µcad,parameter_multiplicity_example_D#todo
use std::geo2d::circle;
use std::translate;

module rounded_rect(width: Length, height: Length, radius: Length) {
    hull()
        translate(x = [-width, width]/2, y = [-height, height]/2)
            circle(radius);
}

module mountable_plate(width: Length, height: Length, corner_radius: Length, distance: Length, hole_diameter = 5mm) {
    rounded_rect(width, height, radius = corner_radius) - {
        hor = (width - distance) / 2;
        ver = (height - distance) / 2;
        translate(x = [-hor, hor], y = [-ver, ver])
            circle(diameter = hole_diameter);
    }
}

module mountable_plate(
    width: Length,
    height: Length,
    corner_radius: Length,
    outer_distance: Length, 
    hole_diameter = 5mm)
{
    plate = rounded_rect(width, height, radius = corner_radius);

    holes = translate(x = [-1, 1] * (width - outer_distance) / 2, 
                      y = [-1, 1] * (height - outer_distance) / 2)
                circle(diameter = hole_diameter);

    plate - holes;
}

module mountable_plate(
    size: (Length, Length),
    corner_radius: Length,
    outer_distance: Length, 
    rel_hole_positions: [(x: Scalar, y: Scalar)], 
    hole_diameter = 5mm)
{
    plate = rounded_rect(width, height, radius = corner_radius);

    holes = translate(hole_positions)
                circle(diameter = hole_diameter);

    plate - holes;
}

module directions {}

namespace hole_positions {
    top_left = (x = -100%, y =  100%);
    north = top_left;
    bottom_left = (x = -100%, y = -100%);
    top_right = (x = 100%, y =  100%);
    bottom_right = (x =  100%, y = -100%);
    top = (x = 0%, y = 100%);
    bottom = (x = 0%, y = -100%);
    left = (x = 100%, y = 0%);
    right = (x = -100%, y = 0%);
    center = (x = 0%, y = 0%);

    corners = [top_left, bottom_left, top_right, bottom_right];

    edges = [top, bottom, left, right];

    all = corners + edges + [center];
}

mountable_plate(
    size = (10cm, 10cm),
    corner_radius = 5mm,
    outer_distance = 1cm,
    hole_positions = hole_positions.edges - [hole_positions.bottom]
);

mountable_plate(size = (10cm, 10cm), corner_radius = 5mm, outer_distance = 1cm,
    [(x = [-100%, 100%], y = [-100%, 100%])]) {
};
```
