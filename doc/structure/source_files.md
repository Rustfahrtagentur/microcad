# Source files

- [Initial Source File](#initial-source-file)
  - [Example 2D Source File](#example-2d-source-file)
  - [Example 3D Source File](#example-3d-source-file)
- [Module Source Files](#module-source-files)

*Source files* are files which include µcad code.

µcad differs between *initial source files* and module *source files*.

## Initial Source File

The initial source file is the one you are starting µcad with.
Initial source files must have some start code which usually initiates the drawing of objects.

If you create objects within this code a workbench will implicitly be created which automatically detects if you generate 2D or 3D objects:

### Example 2D Source File

[![test](.test/initial_source_file_2D.svg)](.test/initial_source_file_2D.log)

```µcad,initial_source_file_2D
// simply draw a circle
std::geo2d::circle(radius = 1cm);
```

### Example 3D Source File

[![test](.test/initial_source_file_3D.svg)](.test/initial_source_file_3D.log)

```µcad,initial_source_file_3D#todo
// simply draw a sphere
std::geo3d::sphere(radius = 1cm);
```

Mixing both will lead to an error:

[![test](.test/initial_source_file_mixed.svg)](.test/initial_source_file_mixed.log)

```µcad,initial_source_file_mixed#fail
std::geo2d::circle(radius = 1cm);
std::geo3d::sphere(radius = 1cm);  // error: can't mix 2D and 3D
```

## Module Source Files

Module *source files* are used from an *initial source file* by using the
[use statement](use.md).

In µcad every file is a module and if you use other files within your initial
source file the start code of those files will be ignored.

But writing some startup code in those files may be useful.
For example you might illustrate what functionalities a file includes by writing
start code which produces images of the objects available in this file.
