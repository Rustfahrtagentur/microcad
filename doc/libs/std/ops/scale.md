# Rotate `std::ops::scale`

We can scale objects in 2D and 3D using `std::ops::scale`:

[![test](.test/scale_uniform.svg)](.test/scale_uniform.log)

```µcad,scale_uniform
std::geo2d::Rect(30mm).std::ops::scale(200%);
```

## Scaling in 3D

[![test](.test/scale_3d.svg)](.test/scale_3d.log)

```µcad,scale_3d
std::geo3d::Cylinder(h = 50mm, d = 20mm).std::ops::scale(x = 200%);
```
