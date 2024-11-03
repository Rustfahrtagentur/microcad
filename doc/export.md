# Export

The builtin function `export()` is available for putting the result of your source code into a file.

The following code writes a cube into an STL file called `cube.stl`:

```µCAD,export
std::export("cube.stl") std::geo3d::cube(size = 40.0mm);
```

Because exporting in µCAD is in code one is able to export several different formats in one run or by conditional selecting.

```µCAD,export_multiple#todo
export(["cube.stl","cube.png"]) cube(40mm);
```

By using attributes the user can access the export methods your code is providing.

```µCAD,export_attribute#todo
#[slider("cube size")]
size = 40mm;

#[export("Export cube as STL")]
export("cube.stl") cube(40mm);

#[export("Export cube as PNG")]
export("cube.png") cube(40mm);
```
