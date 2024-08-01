Each parameter can also be given as a list with elements of the parameter's type.
Each list element will be evaluated. This is called *parameter multiplicity*.
This way, we can intuitively express a call that is called for each parameter variant.

The following example will produce 4 rectangles on different positions:

```µcad
translate(x = [-4.0mm, 4.0mm], y = [-4.0mm, 4.0mm]) 
  rectangle(2.0mm, 2.0mm);
```

The example results in the following calls:

```µcad
translate(x = -4.0mm, y = -4.0mm) rectangle(2.0mm, 2.0mm);
translate(x = -4.0mm, y = 4.0mm) rectangle(2.0mm, 2.0mm);
translate(x = 4.0mm, y = -4.0mm) rectangle(2.0mm, 2.0mm);
translate(x = 4.0mm, y = 4.0mm) rectangle(2.0mm, 2.0mm);
```

Normally, this would require 2 nested for loops:

```ucad
for x = [-4.0mm, 4.0mm] {
  for y = [-4.0mm, 4.0mm] {
 translate(x = x, y = y) 
   rectangle(2.0mm, 2.0mm);
  }
}
```

* `translate(x = [-4.0, 4.0]mm)`

* `translate(x = [-4.0, 4.0] * 1mm)`

* `translate(x = [-1,1] * 4mm)`

```µcad
module rounded_rect(width: length, height: length, radius: length) {
    hull()
        translate(x = [-width, width]/2, y = [-height, height]/2)
            circle(radius);
}

module mountable_plate(width: length, height: length, corner_radius: length, distance: length, hole_diameter = 5mm) {
    rounded_rect(width, height, radius = corner_radius) - {
        horz = (width - distance) / 2;
        vert = (height - distance) / 2;
        translate(x = [-horz, horz], y = [-vert, vert])
            circle(diameter = hole_diameter);
    }
}

module mountable_plate(
    width: length,
    height: length,
    corner_radius: length,
    outer_distance: length, 
    hole_diameter = 5mm)
{
    plate := rounded_rect(width, height, radius = corner_radius);

    holes := translate(x = [-1, 1] * (width - outer_distance) / 2, 
                       y = [-1, 1] * (height - outer_distance) / 2)
                 circle(diameter = hole_diameter);

    plate - holes;
}

```

```
