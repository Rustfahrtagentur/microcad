# Create a sketch for the base

Now, we want to turn the construction of the Lego brick base into a *reusable, parametric component*.
In µcad, a reusable, parametric component that produces or transforms a geometry is called
[*workbench*](../structure/workbench.md).

There are *three* kinds of workbenches:

* [**sketches**](../structure/sketch.md): produce 2D geometry, e.g. `Rect`.
* [**parts**](../structure/part.md): produce 3D geometry, e.g. `Sphere`.
* [**op**](../structure/op.md): Turn some input geometry into output geometry, e.g. `translate`, `union` or `subtract`.

## Definition of our first sketch

Let's *encapsulate* the construction of the frame into a `sketch` workbench called `Base`.

[![test](.test/base.svg)](.test/base.log)

```µcad,base
use std::geo2d::*;
use std::ops::*;

sketch Base(width: Length, height: Length, thickness = 1.2mm) {
    frame = Frame(width, height, thickness);
    struts = Ring(outer_d = 6.51mm, inner_d = 4.8mm)
             .translate(x = [-1..1] * 8mm);
    frame | struts;
}

Base(width = 31.8mm, height = 15.8mm);
```

![Picture](.test/base-out.svg)

If we examine the syntax of the above example, we can see the following things:

* Names of sketches are commonly written in `PascalCase`, starting with a capital letter.
* The sketch `Base` has 3 parameters `width`, `height` and `thickness`. Together they are
  called the *building plan* of `Base`.
* `width` and `height` have the type `Length` and no default value, they are *required*.
* `thickness` is also of type `Length`, but implicitly, because we have defined a default value `1.2mm`
  which is a `Length` of unit `mm`.
* The body `{ ... }` of `Base` constructs the actual geometry.
* `Base(width = 15.8mm, height = 31.8mm)` is a call of the sketch.

And the best part: We don't even need additional value stores for our measures like `thickness`, `width` etc.
Every measure has a meaningful name in the parameters.
This makes the code clearer and changes easier.

### An analogy to natural language

In the previous sections, we have been introduced to main concepts of µcad.
If we draw an analogy to natural language, we can summarize:

* The workbenches `Base`, `Frame` and `Circle` act like a noun, the subject of the sentence -
  it's the geometry being described or manipulated.
* The operation `translate` function like verbs, indicating operations being applied to the geometry.
* The parameters `x = 20mm` and `45°` serve as adverbs, specifying how the operations are carried out.
* Groups `{}` serve as subclauses.
* Assignments `a = Rect(...)` are used to give things a unique name: *`a` is a rectangle*.

This analogy helps illustrate how the µcad syntax is designed to be both readable and logical,
resembling the structure of natural language in a way that makes the code easier to understand.

Now, we have seen all concepts to actually design our Lego brick in 3D.
