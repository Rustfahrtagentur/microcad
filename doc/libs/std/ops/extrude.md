# Extrude

The extrude operation extrudes a 2D geometry into a 3D geometry with a certain height.

[![test](.test/extrude.png)](.test/extrude.log)

```µcad,extrude
std::geo2d::circle(radius = 20mm)
    .std::ops::extrude(height = 20mm);
```
