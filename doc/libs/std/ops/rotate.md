# Rotate `std::ops::rotate`

We can rotate objects in 2D and 3D using `std::ops::rotate`:

[![test](.test/rotate_2d.svg)](.test/rotate_2d.log)

```µcad,rotate_2d
std::geo2d::Rect(30mm).std::ops::rotate(45°);
```

## Rotations in 3D

[![test](.test/rotate_3d.svg)](.test/rotate_3d.log)

```µcad,rotate_3d
cylinder = std::geo3d::cylinder(h = 50mm, d = 20mm);

cylinder.std::ops::rotate(x = 90°);
cylinder.std::ops::rotate(y = 90°);
cylinder.std::ops::rotate(z = 90°);
```
