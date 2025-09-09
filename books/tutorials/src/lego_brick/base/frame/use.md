# Use Statement

You might be wondering why we always have to write `std::geo2d::` and `std::ops::` in front of `Rect` and `subtract`.
This is because builtin sketches (and parts) in µcad are organized within *modules* in a
[standard library](../libs/std/README.md).
`std` is the name of the top module of this *library* and
[`geo2d`](../libs/std/geo2d/README.md)
is a *submodule* of `std` and contains all built-in sketches.

Writing `std::ops` and `std::geo2d` in front of each element seems redundant and cumbersome.
Luckily, µcad has syntax elements called [*use statements*](../structure/use.md).
Apart from the shorter code, another useful feature of the statement is that it allows you to explicitly specify which parts of a module you want to use throughout the source file.
This means instead of the previous code, we can simply write:

[![test](.test/use.svg)](.test/use.log)

```µcad,use
use std::geo2d::Rect;
use std::ops::subtract;

thickness = 1.2mm;
width = 31.8mm;
height = 15.8mm;
{
    Rect(width, height);
    Rect(width = width - 2 * thickness, height = height - 2 * thickness);
}.subtract();
```

![Picture](.test/use-out.svg)

As you can see, this makes the code much simpler and clearer.
