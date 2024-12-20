# Export

The builtin function `export()` is available for putting the result of your source code into a file.

The following code writes a cube into an STL file called `cube.stl`:

![test](.banner/export_single.png)

```µcad,export_single
std::export("cube.stl") std::geo3d::cube(size = 40.0mm);
```

Because exporting in µcad is in code one is able to export several different formats in one run or by conditional selecting.

![test](.banner/export_multiple.png)

```µcad,export_multiple#todo
std::export(["cube.stl","cube.png"]) cube(40mm);
```

By using attributes the user can access the export methods your code is providing.

![test](.banner/export_attribute.png)

```µcad,export_attribute#todo
#[slider("cube size")]
size = 40mm;

#[export("Export cube as STL")]
std::export("cube.stl") cube(40mm);

#[export("Export cube as PNG")]
std::export("cube.png") cube(40mm);
```
