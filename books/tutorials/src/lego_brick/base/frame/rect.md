# Creating a rectangle

To construct a rectangle in µcad, we use a sketch with the name `std::geo2d::Rect`.

Open the `lego_brick.µcad` file you have created before, delete all contents and
replace it with the following single statement:

[![test](.test/rect.svg)](.test/rect.log)

```µcad,rect
std::geo2d::Rect(width = 31.8mm, height = 15.8mm);
```

![Picture](.test/rect-out.svg)

The above statement [calls](../structure/calls.md) a built-in sketch, `std::geo2d::Rect`,
with the parameters `width` and `height` set to our measures.
Like every statement in µcad, it ends with a semicolon (`;`).
Executing this statement will eventually construct the actual geometry.

As you can see, arguments in µcad are quite explicit.
There are **no** positional parameters in µcad!
Instead, arguments must be provided with an identifier or match unambiguously by type.

Also you can see that in µcad all values are attached to a unit (like `mm` in the above code).
The unit defines implicitly the type (e.g. `Length` for `mm`).
If you calculate with types those units will be calculated too.
So if you multiply a length with another you will get an `Area` type (e.g. `mm²`).
