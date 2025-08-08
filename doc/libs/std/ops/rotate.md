# Rotate `std::ops::rotate`

We can rotate objects in 2D and 3D using `std::ops::rotate`:

[![test](.test/rotate_2d.png)](.test/rotate_2d.log)

```µcad,rotate_2d
std::ops::rotate(45°) std::geo2d::rect(30mm);
```

## Rotations in 3D

```µcad,rotate_3d
std::ops::rotate(x = 90°) std::geo3d::cylinder(h = 50mm, d = 20mm);
std::ops::rotate(y = 90°) std::geo3d::cylinder(h = 50mm, d = 20mm);
std::ops::rotate(z = 90°) std::geo3d::cylinder(h = 50mm, d = 20mm);
```
