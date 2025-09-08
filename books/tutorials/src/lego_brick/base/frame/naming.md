# Naming models

During the design process, we will add more geometry to our design.
Therefore, it is useful to identify each sub-geometry by a name.
In the next we want to give the rectangle a name `outer`, so we can identify it more easily:

[![test](.test/assignment.svg)](.test/assignment.log)

```µcad,assignment
outer = std::geo2d::Rect(width = 31.8mm, height = 15.8mm);
```

By adding `outer =` to the call `std::geo2d::Rect(..)`, we have created an [*assignment*](../structure/assignments.md).
Now, the output rectangle will be stored in the variable `outer`.
However, when we export the file again via `microcad export lego_brick.µcad`,
you will notice that nothing is exported.

Why? Because in µcad, assignments are not part of the output geometry.
A second statement `outer;` is needed to output the geometry stored in the `outer` variable.

[![test](.test/output.svg)](.test/output.log)

```µcad,output
outer = std::geo2d::Rect(width = 31.8mm, height = 15.8mm);
outer;
```

![Picture](.test/output-out.svg)

Naming the rectangles leads to some better readability but the code will get quite a bit longer, because
for each rectangle we now need a second statement to render them.

This makes more sense, if you use an *Operator* (`-`) instead of an *Operation* (`subtract()`).

[![test](.test/operator.svg)](.test/operator.log)

```µcad,operator
use std::geo2d::Rect;

thickness = 1.2mm;
width = 31.8mm;
height = 15.8mm;

// name both rectangles
outer = Rect(width, height);
inner = Rect(width = width - 2 * thickness, height = height - 2 * thickness);

// what was { .. }.subtract() before:
outer - inner;
```

![Picture](.test/operator-out.svg)

Now any reader can easily understand what's going on.

As you might mentioned we do not need the line `use std::ops::subtract;` anymore.
This is because µcad brings some builtin binary *operators* which are hard-linked to builtin *operations*:

| Operator | Builtin Operation      | Description              |
| :------: | ---------------------- | ------------------------ |
|   `-`    | `__builtin::subtract`  | Geometrical difference   |
|   `\|`   | `__builtin::union`     | Geometrical union        |
|   `&`    | `__builtin::intersect` | Geometrical intersection |

Those builtin operations are from the *builtin library* which can be found within the global module `__builtin`.

Usually there is no need to use them directly (except with operators), because all builtin
functionalities are also accessible via the *standard library*.
