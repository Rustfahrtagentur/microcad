# µCAD

µcad (pronounced *microcat*) is a declarative programming language for modeling geometric objects.

CSG modeling
Constructing a tree
Modelling

## Examples

## Getting started

### A basic 2D example

When you write a csg file, you basically construct a tree.
Let's assume we can to construct an ISO metric hexagonal nut with the size M10.
Let make a 2D sketch of the nut first:

```csg
// We have to import the primitive2d module to use `hexagon` and `circle` sub-modules
use * from geo2d;

// A generic module for the hex nut
module hex_nut(outer_diameter: length, hole_diameter: length) {
    hexagon(d = self.outer_diameter) - circle(d = self.hole_diameter);
}

// We want to use built-in colors
use colors;

// We want to export our nut as SVG, with blue lines
export("hex_nut.svg", stroke_color = colors.BLUE) {
    hex_nut(11.5mm, 6.0mm);
}
```

Now, we only have 2D version of the nut.
But of course we want to have 3D version!
We can simply generate a 3D model by extruding the nut using the `linear_extrude` operator:

```csg
module hex_nut(outer_diameter: length, inner_diameter: length, height: length) {
    linear_extrude(h = self.height) {
        hexagon(d = self.inner_diameter) - circle(d = self.outer_diameter);
    }
}

// The module `hex_nut` will produce a 3D object that can be exported as STL mesh
export("hex_nut.stl") hex_nut(11.5mm, 3.0mm, 5.0mm);

// We can also export a slice of the nut.
// The operator `slice` will make 2D slice containing polygons from 3D mesh:
export("hex_nut_slice.svg") slice(z = 0.0mm) hex_nut(11.5, 3.0, 5.0mm);
```

Of course now the winding is missing for the nut.
We could construct the winding using `rotate_extrude` operator.
But, even better, there is a built-in module for ISO metric nuts and screws!

```csg
// Import the iso module
use iso;

// Export our nut from STL
export("hex_nut.stl") iso.m10.hex_nut();

// Of course, we can generate the corresponding screw
export("hex_screw.stl") iso.m10.hex_screw(length = 40mm);
```

### A 3D constructive solid geometry example

In our language, we can do constructive solid geometry (CSG).
Let's create a simple cube with a size of 40mm:

```ucad
use cube from geo3d;

cube(size = 40mm);
```

Notice that the `size` parameter name is optional an can be omitted.
We need to export the cube as an STL file.

```ucad
export("cube40mm.stl") cube(40mm);
```

One of the defining features of CSG is the usage of boolean operations on primitives.
Let's create a module for a cube as shown in the image:

```ucad
use * from geo3d;

module csg_cube(size: length) {
    difference() {
        intersection() {
            cube(self.size);
            sphere(r = self.size * math.sqrt(2.0));
        }

        // The list expression `[X,Y,Z]` will make a cylinder for each list item in the respective axis
        // This means no for loop is required!
        orient(axis = [X,Y,Z]) cylinder(d = self.size / 2, h = self.size);
    }
}

export("csg_cube.stl") csg_cube(40mm);
```

You will notice that the usage the boolean operations like `difference` will require lots of brackets and nesting.
Fortunately, we can write this differently without brackets and nesting.
Instead, we will use the `:=` operator to assign a name to each sub-part of the `csg_cube` module, in this case `body` and `axes`.
Moreover, we use the operator `&` and `-` to express the boolean operations:

```csg
module csg_cube(size: length) {
    body := cube(self.size) & sphere(r = self.size / 1.5);
    axes := orient([X,Y,Z]) cylinder(d = self.size / 2, h = self.size * 1.5);

    return body - axes;
}
```

## Conditional statement

A module can define a conditional statement using `if cond {} else {}`:

```csg
use * from geo3d;

module example(size: length) {
    if self.size > 40mm {
        sphere(d = self.size);
    } else {
        cube(self.size);
    }
}
```

### Modules

A module can be defined with arguments:

```ucad
module a(b: length)
```

A module can have initializers:

module a {

}

A module with no arguments and no initializers serves as a namespace.

### Functions

### Documentation

## More examples

* Raspberry Pi case
*

## Compiling