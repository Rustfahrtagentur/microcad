# Export

The built-in function `export()` is available for putting the result of your source code into a file.

The following code writes a cube into an STL file called `cube.stl`:

[![test](.test/export_single.png)](.test/export_single.log)

```µcad,export_single
std::export("cube.stl") std::geo3d::cube(size = 40mm);
```

Because exporting in µcad is in code one is able to export several formats in one run or by conditional selecting.

[![test](.test/export_multiple.png)](.test/export_multiple.log)

```µcad,export_multiple
std::export(["cube.stl","cube.png"]) std::geo3d::cube(40.0mm);
```
