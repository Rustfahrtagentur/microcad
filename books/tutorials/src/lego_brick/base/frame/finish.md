# Finish the frame

Now we have successfully created a model which describes the frame by the difference of two rectangles.

At first glance, it might seem a bit cumbersome to go through multiple steps just to create a simple frame.
However, this example was intentionally designed to introduce you to the fundamental concepts of µcad - such as workbenches,
operations and groups.
Those foundational steps give you a clearer understanding of how µcad works under the hood.

Fortunately, µcad has a ready-to-go sketch to construct a frame geometry like we have constructed
in the `std` library: the `Frame` sketch.
Using it, we can achieve the same result as before but with a much simpler expression:

[![test](.test/frame.svg)](.test/frame.log)

```µcad,frame
// Include all from std::geo2d using * (including Frame)
use std::geo2d::*;

thickness = 1.2mm;
width = 31.8mm;
height = 15.8mm;

// Construct a frame
Frame(width, height, thickness);
```

![Picture](.test/frame-out.svg)

Looks fine, so let's continue with the struts...