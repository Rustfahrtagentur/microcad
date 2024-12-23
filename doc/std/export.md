# Export

The builtin function `export()` is available for putting the result of your source code into a file.

The following code writes a cube into an STL file called `cube.stl`:

![test](.test/export_single.png)
[see build log](.test/export_single.log)

```µcad,export_single
std::export("cube.stl") std::geo3d::cube(size = 40.0mm);
```

Because exporting in µcad is in code one is able to export several different formats in one run or by conditional selecting.

![test](.test/export_multiple.png)
[see build log](.test/export_multiple.log)

```µcad,export_multiple
std::export(["cube.stl","cube.png"]) std::geo3d::cube(40.0mm);
```

By using attributes the user can access the export methods your code is providing.

![test](.test/export_attribute.png)
[see build log](.test/export_attribute.log)

```µcad,export_attribute#todo
#[slider("cube size")]
size = 40mm;

#[export("Export cube as STL")]
std::export("cube.stl") cube(40mm);

#[export("Export cube as PNG")]
std::export("cube.png") cube(40mm);
```
