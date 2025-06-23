# Statements

- [Available Statements](#available-statements)
- [Code Bodies](#code-bodies)
  - [Source files](#source-files)
    - [Initial Source File](#initial-source-file)
      - [Example 2D Source File](#example-2d-source-file)
      - [Example 3D Source File](#example-3d-source-file)
    - [Module Files](#module-files)
  - [Functions](#functions)
  - [Workbenches](#workbenches)

## Available Statements

- [Use Statements](use.md) import stuff from other *modules*
- [Assignments](assignments.md) assign *values* to *variables* and *properties*
- [Functions](functions.md) separate functionality with an own *code body*
- [Calls](calls.md) call *workbenches*, *functions* or *builtin functions*
- [Conditionals](conditionals.md) like `if`/`else`

## Code Bodies

The following entities have a *code body* and so may implement running code which consists of a list of *statements*.

### Source files

In general every source file can have a code body.

#### Initial Source File

The initial source file is the one you are starting µcad with.
Initial source files must have some start code which usually initiates the drawing of objects.

If you create objects within this code a workbench will implicitly be created which automatically detects if you generate 2D or 3D objects:

##### Example 2D Source File

[![test](.test/initial_source_file_2D.png)](.test/initial_source_file_2D.log)

```µcad,initial_source_file_2D
// simply draw a circle
std::geo2d::circle(radius = 1cm);
```

##### Example 3D Source File

[![test](.test/initial_source_file_3D.png)](.test/initial_source_file_3D.log)

```µcad,initial_source_file_3D
// simply draw a cube
std::geo3d::sphere(radius = 1cm);
```

Mixing both will lead to an error:

[![test](.test/initial_source_file_mixed.png)](.test/initial_source_file_mixed.log)

```µcad,initial_source_file_mixed#fail
std::geo2d::circle(radius = 1cm);
std::geo3d::sphere(radius = 1cm);  // error: can't mix 2D and 3D
```

#### Module Files

In µcad every file is a module and if you use other files within your initial source file the start code of those files will be ignored.

But writing some startup code in those files may be useful.
For example you might illustrate what functionalities a file includes by writing start code which produces images of the objects available in this file.

### Functions

The code body of functions is the part within the curly braces of the function definition.

Functions may return a value as result:

[![test](.test/function_code.png)](.test/function_code.log)

```µcad,function_code
fn pow( x: Scalar, n: Integer ) {
    if n == 1 {
        x   // return x
    } else {
        x * pow(n-1) // return recursive product
    }
}
```

### Workbenches

Workbenches may have *init code* and *building code* (see [workbench description](workbench.md)) which do consist of statements.
