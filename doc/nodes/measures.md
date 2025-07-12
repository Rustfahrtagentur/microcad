# Builtin Measures

*Measures* are builtin methods you can use with 2D or 3D objects.

The following example calculates the area of a circle by using the *measure* `area`:

[![test](.test/measure.png)](.test/measure.log)

```µcad,measure
__builtin::assert_eq([

    // use measure area() on a circle
    std::geo2d::circle(radius=10mm).area(),

    // circle area formula for comparison
    10mm * 10mm * std::math::PI
]);
```

Currently it is not possible to declare measures in µcad.

## 2D Measures

Builtin measures that can be used on 2D objects.

| Measure      | Output Quantity                                          | Description         |
| ------------ | -------------------------------------------------------- | ------------------- |
| `area(..)`   | `Area`                                                   | area                |
| `circum(..)` | `Length`                                                 | circumference       |
| `center(..)` | `(x:Length, y:Length)`                                   | geometric center    |
| `size(..)`   | `(width:Length, height:Length)`                          | extents             |
| `bounds(..)` | `(left:Length, right:Length, top:Length, bottom:Length)` | bound (from center) |

## 3D Measures

Builtin measures that can be used on 3D objects.

| Measure      | Output Quantity                                                                       | Description         |
| ------------ | ------------------------------------------------------------------------------------- | ------------------- |
| `area(..)`   | `Area`                                                                                | surface area        |
| `center(..)` | `(x:Length, y:Length, z:Length)`                                                      | geometric center    |
| `size(..)`   | `(depth:Length, width:Length, height=Length)`                                         | extents             |
| `bounds(..)` | `(front: Length, back: Length, left:Length, right:Length, top:Length, bottom:Length)` | bound (from center) |
| `volume(..)` | `Volume`                                                                              | volume              |
