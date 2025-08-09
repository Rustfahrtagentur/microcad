# Revolve

The revolve operation revolves a 2D geometry into a 3D geometry.

[![test](.test/revolve.svg)](.test/revolve.log)

```µcad,revolve
// Construct half of a torus. 
std::geo2d::circle(radius = 10mm)
    .std::ops::translate(x = 40mm)
    .std::ops::revolve(180°);
```
