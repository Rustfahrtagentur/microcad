# Creating a second rectangle

Like the outer frame, the inner frame is a `std::geo2d::Rect` too:

[![test](.test/inner.svg)](.test/inner.log)

```µcad,inner
thickness = 1.2mm;
std::geo2d::Rect(width = 31.8mm - 2 * thickness, height = 15.8mm - 2 * thickness);
```

![Picture](.test/inner-out.svg)

We have defined a new value `thickness = 1.2mm` to store the frame's wall thickness.
Then, we construct a rectangle by taking the original width and height and subtracting
twice the `thickness` from both.

We can now output the inner and outer geometry simultaneously.
Similar to the `thickness = 1.2mm`, we also assign `width` and `height` their respective
values to shorten the code:

[![test](.test/inner_outer.svg)](.test/inner_outer.log)

```µcad,inner_outer
thickness = 1.2mm;
width = 31.8mm;
height = 15.8mm;
std::geo2d::Rect(width, height);
std::geo2d::Rect(width = width - 2 * thickness, height = height - 2 * thickness);
```

![Picture](.test/inner_outer-out.svg)

Because the arguments we give to the first `std::geo2d::Rect()` match exactly the parameter
names of it we do not need to write extra parameter names here.
This is called *auto-matching*.
It prevents us from having to write the argument names twice:

```µcad
std::geo2d::Rect(width = width, height = height);
```

Now, we can execute the export command from the command line tool:

```sh
microcad export lego_brick.µcad
```

The `export` command will produce a [*Scalable Vector Graphic* (SVG)](https://en.wikipedia.org/wiki/SVG) file named `lego_brick.svg`
next to the `lego_brick.µcad` file.
By default, all 2D geometries are exported to *SVG*.

Congratulations, you have exported your first 2D geometry with µcad!

Although the measurements of these rectangles are correct, our intention was to create a
frame in which the both rectangles define the boundary of the frame.
To achieve this, we will use an *operation* to combine them.
