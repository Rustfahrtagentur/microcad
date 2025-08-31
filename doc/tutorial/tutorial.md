# µcad Tutorial

## Intro

This tutorial will introduce you to the basic concepts of the µCAD language.
The goal is to create a parametric Lego brick part and provide it as a reusable library.
Basic programming and CAD knowledge are required to complete this tutorial.

## Getting started

* Install VSCode: [https://code.visualstudio.com/]
  * Install the STL Viewer extension: [https://open-vsx.org/extension/mtsmfm/vscode-stl-viewer]
  * Install the µcad extension.
* Install microcad: `cargo install microcad`

## Overview

This tutorial is split in four parts:

1) Explaining how to create basic geometries in 2D.
2) Creating a sketch in 2D.
3) Create a Lego brick part in 3D.
4) Making the Lego brick parametric and reusable.

### Constructing a 4x2 Lego brick

Before designing a fully customizable Lego brick, we first want to construct a brick of fixed size.
A Lego brick essentially consists of three components:

* *Base*: A rectangular frame with struts.
* *Cap*: A rectangular top plate that closes the base structure.
* *Knobs*: The knobs of a Lego brick placed on top of the cap.

Before constructing the entire 3D model, we design the three components as 2D sketches.
After that, we will use them to construct a 3D part, that will be put into a library.
In µcad, designs of 2D geometries are called [*sketches*](../structure/sketch.md) while designs 3D geometries are called [*parts*](../structure/part.md) .

### Creating a new µcad file

Before we design any geometry, we use the `microcad` command line interface to create a new µcad project:

```sh
microcad create lego_brick
```

This will create a file `lego_brick.µcad`. Let's open this file in VSCode.
It will export a circle and print `Hello µcad`.

If we export the file with the command:

```sh
microcad export lego_brick
```

Nothing will be exported because the sketch does not contain any output geometry.
Therefore, let's add some geometry!

## A basic 2D geometry: The frame

The first geometry we want to construct is the frame of the brick.
It consists of an outer and an inner rectangle.

### Frame outer

The outer part of the frame is a rectangle with `width = 15.8mm` and `height = 31.8.mm`.
To construct a rectangle in µcad, we use a sketch with the name `std::geo2d::Rect`.

In the `lego_brick.µcad` file, delete all contents and replace it with the following statement:

```µcad,tutorial_2d_rect
std::geo2d::Rect(width = 15.8mm, height = 31.8mm);
```

The statement above is a [call](../structure/calls.md) to sketch `std::geo2d::Rect` with the parameters `width` and `height`.
In µcad, each statement is finished `;`.
The execution of this statement will eventually the construct the actual geometry.

Now, we can execute the export command from the CLI again:

```sh
microcad export lego_brick.µcad
```

The command will produce an SVG file `lego_brick.svg` next to the `lego_brick.µcad` file.
By default, all 2D geometries are exported to SVG.
Congratulations, you have exported your first 2D geometry with µcad!

You might be wondering why we have to write `std::geo2d::Rect` instead of just `Rect`.
This is because sketches and parts in µcad are organized in modules and library.
`std` is the name for the [µcad standard library](../libs/std/README.md) which contains all built-in workbenches.
[`geo2d`](../libs/std/geo2d/README.md) is a submodule of `std` and contains all built-in sketches.

### Naming things: *Assignments*

During the design process, we will add more geometry to our design.
Therefore, it is useful to identify each sub-geometry by a name.
In the next we want to give the rectangle a name `frame_outer`, so we can identify it more easily:

```µcad,tutorial_2d_assignment
frame_outer = std::geo2d::Rect(width = 15.8mm, height = 31.8mm);
```

By adding `frame_outer =` to the call `std::geo2d::Rect(..)`, we have created an [*assignment*](../structure/assignments.md).
Now, the output rectangle will be stored in the variable `frame_outer`.
However, whhen we export the file again via `microcad export lego_brick.µcad`,
you will notice that nothing is exported.

Why? Because in µcad, assignments are not part of the output geometry.
A second statement `frame_outer;` is needed to output the geometry stored in the `frame_outer` variable.

```µcad,tutorial_2d_output
frame_outer = std::geo2d::Rect(width = 15.8mm, height = 31.8mm);
frame_outer;
```

The output should be same as the previous SVG we have exported before.
Now, let's create the inner frame of the brick base.

### Construct the inner frame

Like the outer frame, the inner frame is also expressed with a `std::geo2d::Rect`.
Let's take a look at the code:

```µcad,tutorial_2d_inner
wall_width = 1.2mm;
frame_inner = std::geo2d::Rect(width = 15.8mm - 2*wall_width, height = 31.8mm - 2*wall_width);
frame_inner;
```

In the above source code snippet, we have defined a new value `wall_width = 1.2mm` to store the frame's wall width.
Next, we construct a new rectangle by taking the original width and height and subtracting twice `wall_width`.
This subtraction shrinks the rectangle on each side.
Finally, we output the geometry with the `frame_inner;` statement.

### Output both inner and outer frame

Now, we are able to output the inner and outer geometry at the same time.
Similar to the `wall_width = 1.2mm`, we also assign `width` and `height` their respective values:

```µcad,tutorial_2d_inner_outer
wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;
frame_outer = std::geo2d::Rect(width, height);
frame_inner = std::geo2d::Rect(width = width - 2*wall_width, height = height - 2*wall_width);
frame_outer;
frame_inner;
```

This time, when you re-export the geometries, you will see that there are two rectangles in the resulting SVG.
Although the measurements of these rectangles are correct, our intention was to create a frame where `frame_outer` defines the outer boundary and `frame_inner` defines the inner boundary (*hole*) in the frame.
To achieve this, we will use *operations*. Let's take a brief detour to discuss this topic.

### Manipulate geometry with *Operations*

[Operations](../structure/op.md) are one of µcad's most essential features.
They process an input geometry and produce an output geometry.
All operations in the µcad standard library `std` are located in the submodule [`ops`](../libs/std//ops/README.md).
Consider the following example that translates a `Rect` with a size of `40mm` by `20mm` in `x` direction:

```µcad,tutorial_2d_translate
std::geo2d::Rect(40mm).std::ops::translate(x = 20mm);
```

Let's examine the syntax of the above example.
First, we construct a rectangle with `std::geo2d::Rect(40mm)`, and then we translate in `x` direction `x = 20mm` with [`std::ops::translate(x = 20mm)`](../libs/std/ops/translate.md).
We apply the operation using the `.`. This way we can also apply several operations to a geometry:

```µcad,tutorial_2d_translate_rotate
std::geo2d::Rect(40mm)
    .std::ops::translate(x = 20mm)
    .std::ops::rotate(45°);
```

We construct a rectangle, then we translate it, and finally, we [`rotate`](../libs/std/ops/rotate.md) it by `45°`.

### Use statements

You might have noticed that writing `std::ops` and `std::geo2d` in front of each element seems redundant and cumbersome.
Luckily, µcad has syntax element called [*use statements*](../structure/use.md).
This means, from the previous example, we can simply write:

```µcad,tutorial_2d_use
use std::geo2d::Rect; // Only include `Rect` from `std::geo2d`.
use std::ops::*; // Include all operations from `std::ops`.

Rect(40mm)
    .translate(x = 20mm)
    .rotate(45°);
```

As you can see, this makes the code much simpler and clearer.

### Grouping Statements with `{}` in µcad

Let's get back to our actual task to construct a frame.
Here is how source code looks with use statements:

```µcad,tutorial_2d_inner_outer_use
use std::geo2d::Rect;

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;
frame_outer = Rect(width, height);
frame_inner = Rect(width = width - 2*wall_width, height = height - 2*wall_width);
frame_outer;
frame_inner;
```

Another useful syntax feature in µcad is the ability to group several statements into a single output using curly brackets `{}`.
Let's change the code in `lego_brick.µcad`:

```µcad,tutorial_2d_groups
use std::geo2d::Rect;

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;

frame = {
    outer = Rect(width, height);
    inner = Rect(width = width - 2*wall_width, height = height - 2*wall_width);
    outer;
    inner;
};
frame;
```

Notice that we can omit the `frame_` prefix for the `outer` and `inner` variables.
These values are **local** to the `{}` block and won't be accessible from outside.
This helps keep the global namespace clean and avoids unnecessary clutter.

When you re-export the file, you'll see that the visual output remains unchanged—only the structure of the code has improved.

### Applying an operation to a group

Now, we want to combine the `outer` and `inner` geometry using an operation.
We have already seen the operations `translate` and `rotate`.
In µcad, the operation to subtract a geometry from one another is called [`difference`](../libs/std/ops/difference.md).
On our case, we want to subtract the outer part by the inner part in our frame group:

```µcad,tutorial_2d_difference
use std::geo2d::Rect;

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;

frame = {
    outer = Rect(width, height);
    inner = Rect(width = width - 2*wall_width, height = height - 2*wall_width);
    outer;
    inner;
}.std::ops::difference(); // Apply the operation.

frame;
```

### The `-` operator

You might notice that subtracting one geometry from another can be written just like a mathematical subtraction using the `-` operator.
In addition to *operations*, µcad supports *operators*, which lead to clearer and more concise code:

```µcad,tutorial_2d_operator
use std::geo2d::Rect;

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;

frame = {
    outer = Rect(width, height);
    inner = Rect(width = width - 2*wall_width, height = height - 2*wall_width);
    outer - inner; // Use binary operator `-` instead of `std::ops::difference()`
};

frame;
```

Here, we're using the binary `-` operator to subtract the inner rectangle from the outer, effectively creating a rectangular frame.
This syntax replaces the more verbose `std::ops::difference()` operation, making the code easier to read.

When you re-export the file, you’ll see that the resulting shape is a clean rectangular frame, just as expected.

### The power of `std`: Using `std::geo2d::Frame`

At first glance, it might seem a bit cumbersome to go through multiple steps just to create a simple frame.
However, this example was intentionally designed to introduce you to the fundamental concepts of µcad -- such as workbenches,
operations and groups.
These foundational steps give you a clearer understanding of how µcad works under the hood.

Fortunately, µcad’s to construct a frame geometry, the `std` library provides a convenient shortcut: the `Frame` sketch.
Using it, we can achieve the same result with a much simpler expression:

```µcad,tutorial_2d_frame
use std::geo2d::*; // Include all modules from `std::geo2d`

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;

frame = Frame(width, height, thickness = wall_width); // Construct a frame
```

### Summary: An analogy to natural language

In the previous sections, we have been introduced to main concepts of µcad.
If we draw an analogy to natural language, we can summarize:

* The workbench `Rect` acts like a noun, the subject of the sentence -- it's the geometry being described or manipulated.
* The operations `translate` and `rotate` function like verbs, indicating operations being applied to the geometry.
* The parameters `x = 20mm` and `45°` serve as adverbs, specifying how the operations are carried out.
* Groups `{}` serve as subclauses.
* Assignments `a = Rect(...)` are used to give things a unique name: `"a" is a rectangle`.

This analogy helps illustrate how the µcad syntax is designed to be both readable and logical, resembling the structure of natural language in a way that makes the code easier to understand.

## Creating our first `sketch`

In the previous chapter, we explored how to construct basic geometries and combine them using operations like subtraction and translation.
In this chapter, we’ll finish the base geometry for our Lego brick and turning it into a reusable component.
Along the way, we’ll introduce several new µcad concepts that will help make our designs more flexible, concise, and reusable.

### Constructing single a strut with `std::geo2d::Circle`

A single strut has a circular shape with an outer diameter of `d = 6.51mm` and an inner diameter of `d = 4.8mm`.
We use [`std::geo2d::Circle`](../libs/std/geo2d/README.md#Circle) to construct the circles and similar the frame geometry in the previous chapter, we combine them using the `-` operator:

```µcad,tutorial_sketch_begin
use std::geo2d::*;
Circle(d = 6.51mm) - Circle(d = 4.8mm)
```

### Constructing multiple struts

Instead of constructing only a single strut, but our Lego brick needs *three* of them.
The shape of each strut remains the same -- they’re simply offset vertically by `8mm`.
From the concept we know already, a first solution could be to write the strut three times and translating them using `std::ops::translate`.

```µcad,tutorial_sketch_multiple
use std::geo2d::*;
use std::ops::*;

Circle(d = 6.51mm) - Circle(d = 4.8mm);
(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = 8mm);
(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = -8mm);
```

The code above produces the expected result. However, the code is quite repetitive.
We could improve it slightly by storing the expression `Circle(d = 6.51mm) - Circle(d = 4.8mm)` in a variable, say `strut`.
But even then, we’d still need to write out each `translate(...)` call manually.

And more importantly:
What if we wanted the number of struts to be flexible or generated dynamically?

To solve this in a clean and scalable way, µcad supports *multiplicity*, allowing us to generate repeated geometry with minimal, reusable code.

Let’s explore that next.

### Multiplicity

To avoid having to call `translate` multiple times, µcad provides a powerful feature called *multiplicity*.
Instead of applying `translate` separately for each position, you can pass an [array of values](../types/arrays.md).
An array of values is expressed with `[]` brackets.

µcad will automatically apply the operation once for each value in the array.

This allows us to shorten the previous example significantly:

```µcad,tutorial_sketch_multiplicity
use std::geo2d::*;
use std::ops::*;

(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = [-8mm, 0mm, 8mm]);
```

With just a single line of code, we've created three struts—each correctly positioned!
This approach is not only more concise, but also easier to maintain and scale,
especially if you later want to add more positions dynamically.

### Constants

Across the Lego universe, the spacing of `8mm` is used everywhere.
This means we can store it in global constant using the `const` keyword.

```µcad,tutorial_sketch_constants
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = [-SPACING, 0mm, SPACING]);
```

We still have to write `SPACING` twice, but we can change this be multiplying the array:

```µcad,tutorial_sketch_array_multiplication
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = [-1, 0, 1] * SPACING);
```

### Creating arrays with range expressions

The term `[-1, 0, 1]` can be replaced with a range expression `[0..3]`, which yields an array `[0, 1, 2]`:

```µcad,tutorial_sketch_creating_arrays
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

(Circle(d = 6.51mm) - Circle(d = 4.8mm)).translate(y = [0..3] * SPACING);
```

A range expression has the syntax `[m..n]` where `m` and `n` have to be of type `Integer`.

However, after changing the file you can see that the struts are not centered anymore.
To center the struts, we can apply the `std:ops::center` operation:

```µcad,tutorial_sketch_range_expressions
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

(Circle(d = 6.51mm) - Circle(d = 4.8mm))
    .translate(y = [0..3] * SPACING)
    .center();
```

At this point, we are almost finished: We just have to find a way to combine frame and structs.

### Boolean operations

We have to combine the frame and the struts into a single geometry using the [`union`](../libs//std//ops//union.md) operation.
Similar to the `difference` operation, that can be expressed using the `-` operator, the `union` operation be expressed via the `|` operator.
The `difference` and `union` are called *boolean operations*. You can read more about boolean operation in the documentation.

The code in the `lego_brick.µcad` with the whole 2D base geometry of the brick will look like this:

```µcad,tutorial_sketch_boolean_operations
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

wall_width = 1.2mm;
width = 15.8mm;
height = 31.8mm;
frame = Frame(width, height, thickness = wall_width);
struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
    .translate(y = [0..3] * SPACING)
    .center();

frame | struts;
```

If you export the file, you will see a frame and the structs combined into a single object.

### Make it reusable: *Workbenches*

Now, we want to turn the construction of the Lego brick base into a *reusable, parametric component*.
In µcad, a reusable, parametric component that produces or transforms a geometry is called [*workbench*](../structure/workbench.md).
There *three* kinds of workbenches:

* [**sketches**](../structure/sketch.md): produce 2D geometry (e.g. a `Rect`).
* [**parts**](../structure/part.md): produce 3D geometry, e.g. `Sphere`. We will create a `part` for our final Lego brick.
* [**op**](../structure/op.md): Turn some input geometry into an output geometry, e.g. `translate`, `difference`.

### Definition of our first sketch `Base`

Now, we want to *encapsulate* the construction of the frame into a `sketch` workbench called `Base`.

```µcad,tutorial_sketch_base
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length, wall_width = 1.2mm) {
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .translate(y = [0..3] * SPACING)
        .center();
    frame | struts;
}

Base(width = 15.8mm, height = 31.8mm);
```

If we examine the syntax of the above example, we can see the following things:

* Names of sketches are commonly written in `PascalCase`, starting with a capital letter.
* The sketch `Base` has 3 parameters `width`, `height` and `wall_width`. These parameters are called *building plan*.
* `width` and `height` have the type `Length` and no default value, they are *required*.
* `wall_width` is also of type value, but implicitly, because we have a default value `1.2mm`.
* The body `{ ... }` of `Base` constructs the actual geometry.
* `Base(width = 15.8mm, height = 31.8mm)` is a call of the sketch.

Now, we have seen all concepts to actually design our Lego brick in 3D.

## Constructing a `part`: Going 3D

With the knowledge we have gained in the previous chapter,
the remaining sections of the brick -- `Cap` and `Knobs` -- can be constructed swiftly.

### Cap: name aliases

The `Cap` is nothing more than a rectangle. We do not have to define a specific `sketch Cap`, instead we can use *aliases*:

```µcad,tutorial_part_cap
use std::geo2d::Rect as Cap;

width = 15.8mm;
height = 31.8mm;

Cap(width, height);
```

`Cap` will now be known as `Rect`.

### Knobs

The knobs of the brick are simple circles with a diameter of `4.8mm`.
We can easily construct a grid with circles via multiplicity:

```mcad,tutorial_part_circle
const SPACING = 8mm;

std::geo2d::Circle(d = 4.8mm, c = (x = [0..2] * SPACING, y = [0..4] * SPACING));
```

The will 2x4 in horizontal and vertical direction, respectively.

### Tuples

Notice that we have called the `std::geo2d::Circle` with additional argument `c`.
`c` is given as a tuple `(x = ..., y = ...)`. A [tuple](../types/tuples.md) is a collection of (mostly named) values.
The parameter `c` of a circle is supposed to be a tuple of type `(x: Length,y: Length)`.
If we pass to array of `Length` to the tuple, we can generate a multiplicity, which eventually creates `2*4` circles.

```µcad,tutorial_part_knobs
const SPACING = 8mm;

sketch Knobs(diameter = 4.8mm) {
    std::geo2d::Circle(d = 4.8mm, c = (x = [0..2] * SPACING, y = [0..4] * SPACING))
        .center();
}

Knobs();
```

### Overview of the sketches

Below is an intermediate result of the sketches of the three components we now have defined:

```µcad,tutorial_part_overview
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length) {
    use std::geo2d::*;
    wall_width = 1.2mm;
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .translate(y = [0..3] * SPACING)
        .center();
    frame | struts;
}

use std::geo2d::Rect as Cap;

sketch Knobs() {
    Circle(d = 4.8mm, center = (x = [0..2] * SPACING, y = [0..4] * SPACING)).center();
}

width = 15.8mm;
height = 31.8mm;

Base(width, height);
Cap(width, height);
Knobs();
```

In the next steps, we want to create a 3D geometry.

### A workbench for 3D geometry: `part`

Like a sketch, a [*part*](../structure/part.md) is also workbench but calling a part produces a 3D geometry.

### Default parts in the `std`: `std::geo3d`

There are several standard 3D parts in the standard library in the [`std::geo3d`](../libs/std/geo3d/README.md) submodule.
The example below constructs a dice with holes in each direction:

```µcad,tutorial_part_geo3d
use std::math::*;
use std::ops::*;
use std::geo3d::*;

size = 40mm;
body = Sphere(r = size / 1.5) & Cube(size);
holes = Cylinder(h = size, d = size / 1.5).orient([X,Y,Z]);
body - holes;
```

Using the `std::geo3d` is suitable way of 3D geometries directly.
However, we want to create a 3D geomety by extruding a 2D geometry.

### Extrusion

We want to convert the three 2D sketches into a 3D geometry part.
This can be achieved by *extrusion* and the corresponding µcad operation is called [`std::ops::extrude`](../libs/std/ops/extrude.md).

As a first example, let's take the cap of brick and extrude with a `height = 1.0mm`.

```µcad,tutorial_part_extrude_cap
use std::geo2d::Rect as Cap;
use std::ops::extrude;

width = 15.8mm;
height = 31.8mm;

Cap(width, height)
    .extrude(height = 1.0mm);
```

This will create a box with dimensions `15.8mm * 31.8mm * 1.0mm`.
Notice that `height = 1.0` will extrude will in Z axis.

### A first dreft for a parametric part

Let's create a first draft for the brick by taking the base sketch and using it in the `LegoBrick` part:

```µcad,tutorial_part_base
use std::geo2d::*;
use std::ops::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length) {
    wall_width = 1.2mm;
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .translate(y = [0..2] * SPACING)
        .center();
    frame | struts;
}

part LegoBrick(base_height = 9.6mm) {
    width = 15.8mm;
    height = 31.8mm;

    base = Base(width, height)
        .extrude(base_height);

    base;
}

LegoBrick(); // Instantiate a Lego brick
```

### A `lego_brick` part definition

Now, let's also consider the `Base` and the `Cap` sketch and also integrate it into the part.
We extrude `Base`, `Knobs` and `Cap` and translate it in Z direction if necessary.
Afterwards, we combine the three components by the `|` operator.

```µcad,tutorial_part_result
use std::ops::*;
use std::geo2d::*;

const SPACING = 8mm;

sketch Base(width: Length, height: Length) {
    wall_width = 1.2mm;
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .translate(y = [0..2] * SPACING)
        .center();
    frame | struts;
}

use Rect as Cap;

sketch Knobs() {
    Circle(diameter = 4.8mm, center = (x = [0..2] * SPACING, y = [0..4] * SPACING)).center();
}

part LegoBrick(base_height = 9.6mm) {
    width = 15.8mm;
    height = 31.8mm;
    top_thickness = 1.0mm;

    base = Base(width, height)
        .extrude(base_height);

    cap = Cap(width, height)
        .extrude(top_thickness)
        .translate(z = base_height - top_thickness);

    knobs = Knobs()
        .extrude(1.7mm)
        .translate(z = base_height);

    base | cap | knobs; // Combine all components
}

LegoBrick();
```

When we take the code snippet above and export it, instead of an SVG, an STL file will be exported:

```sh
microcad export lego_brick
```

### Summary: A first part design

In the previous chapter, we have created our first 3D part by extruding 2D sketches.
We now have explained the basic µcad feature set, that allows us to create 3D objects.

The next chapter will explain more advanced features in order to create a library with fully parametric Lego brick.

## A library for fully parametric Lego brick

Now we want to make the Lego brick part fully parametric and reusable.
We want to control the number of knobs in both directions, and its height as well.
Moreover, we want to create a reusable Lego brick library.
Therefore, we will introduce the following concepts in the chapter:

* Custom operations with `op`.
* Controlling the export of geometries by *attributes*.
* Creating a simple library.

### A custom operation: `op`

The knobs and struts are created using multiplicity and the `center` operation.
We can put both operations in a custom operation called `center_grid`:

```µcad
const SPACING = 8mm;

op center_grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .center()
}
```

The `center_grid` operation takes `rows` and `columns` as parameters.
When can then rewrite `Knobs` and `Frame` sketches by adding `rows` and `columns` as parameter and using the `center_grid` operation:

```µcad
sketch Base(
    rows: Integer,
    columns: Integer,
    width: Length,
    height: Length
) {
    wall_width = 1.2mm,
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .center_grid(rows = rows - 1, columns = columns - 1);
    frame | struts;
}

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm).center_grid(rows, columns);
}
```

### The final part definition

Additionally, to the `center_grid` operation, we have to compute the overall and width and height in the `LegoBrick` part:

* `width = rows * SPACING - 0.2mm`
* `height = columns * SPACING - 0.2mm`

Now we have achieved the final result:

```µcad
use std::ops::*;
use std::geo2d::*;

const SPACING = 8mm;

op center_grid(rows: Integer, columns: Integer) {
    @children
        .translate(x = [0..rows] * SPACING, y = [0..columns] * SPACING)
        .center()
}

sketch Base(rows: Integer, columns: Integer, width: Length, height: Length) {
    wall_width = 1.2mm;
    frame = Frame(width, height, thickness = wall_width);
    struts = (Circle(d = 6.51mm) - Circle(d = 4.8mm))
        .center_grid(rows = rows - 1, columns = columns - 1);
    frame | struts;
}

use Rect as Cap;

sketch Knobs(rows: Integer, columns: Integer) {
    Circle(d = 4.8mm).center_grid(rows, columns);
}

part LegoBrick(rows: Integer, columns: Integer, base_height = 9.6mm) {
    width = rows * SPACING - 0.2mm;
    height = columns * SPACING - 0.2mm;
    cap_thickness = 1.0mm;

    base = Base(rows, columns, width, height)
        .extrude(base_height - cap_thickness);

    cap = Cap(width, height)
        .extrude(cap_thickness)
        .translate(z = base_height - cap_thickness);

    knobs = Knobs(rows, columns)
        .extrude(1.7mm)
        .translate(z = base_height);

    base | cap | knobs;
}

// Let's create a few bricks with different parameters

LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2)
    .translate(y = -40mm);

LegoBrick(rows = 4, columns = 2);

LegoBrick(rows = 3, columns = 2, base_height = 3.2mm)
    .translate(y = 40mm);
```

### Exploring multiple STL file

We can export the whole the geometry as an STL:

```sh
microcad export lego_brick
```

You will notice that all Lego brick are located in the same file.
We can annotate `#[export]` attributes to export each brick to a different file:

```µcad
#[export = "lego_brick2x2.stl"]
brick2x2 = LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2);

#[export = "lego_brick4x2.stl"]
brick4x2 = LegoBrick(rows = 4, columns = 2);

#[export = "lego-brick5x1.stl"]
brick3x2 = LegoBrick(rows = 5, columns = 1, base_height = 3.2mm);
```

When we export the file now, three files with the specified names will be created.

### Making a Lego brick module

Let's assume we want to use the Lego_brick as library.
Fortunately, this is simple!
We create a second file `my_brick.µcad`:

```sh
microcad create my_brick
```

The directory structure is supposed to contain these files:

```sh
lego_brick.µcad
my_brick.µcad
```

Let's add the following content to the `my_brick.µcad` file:

```µcad
use lego_brick::LegoBrick;

LegoBrick(rows = 3, columns = 1);
```

We include the `lego_brick.µcad` file via the `use` statement.
Then we are ready to use the `LegoBrick` part.
That's all!

### Summary

In this tutorial, we have created a library containing a parametric, reusable Lego brick part.
Of course, the tutorial did not show the full feature set.
For further reading, we recommend reading the [documentation](../README.md).
