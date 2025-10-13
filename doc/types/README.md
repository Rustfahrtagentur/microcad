# Types

The µcad type system consists of a group built-in types.
The type system is static, which means a declared variable has a fixed type that cannot be changed.

These classes of built-in types are supported:

| Built-in type                           | Description                                             | Example                                          |
| --------------------------------------- | ------------------------------------------------------- | ------------------------------------------------ |
| [*Quantity*](quantity.md)               | Numeric values with an optional unit.                   | `a: Length = 4mm`                                |
| [*Bool*](primitive_types.md#bool)       | A boolean value.                                        | `b: Bool = true`                                 |
| [*Integer*](primitive_types.md#integer) | An integer value without a unit.                        | `c: Integer = 4`                                 |
| [*String*](primitive_types.md#string)   | A string.                                               | `d: String = "Hello World"`                      |
| [*Matrix*](primitive_types.md#matrix)   | Matrix types for affine transforms, for internal usage. | ---                                              |
| [*Array*](arrays.md)                    | A list of values with a *common type*.                  | `e: [Integer] = [1,2,3]`                         |
| [*Tuple*](tuples.md#tuples)             | A list of values with a *distinct types*.               | `f: (Length, Scalar) = (4mm, 4.0)`               |
| [*Named tuple*](tuples.md#named-tuples) | A sorted list of key-value pairs with *distinct types*. | `g: (x: Scalar, y: Length) = (x = 4.0, y = 4mm)` |
| [*Models*](nodes.md)                    | Nodes in the model tree.                                | `h: Models = { Cube(2mm); }`                     |

## Declaration

The examples in the table above declare the type explicitly.
However, we can use units to declare the implicitly.
Using units is recommended and what you get in return is that declarations are quite handy:

[![test](.test/types_def_vs_decl.svg)](.test/types_def_vs_decl.log)

```µcad,types_def_vs_decl
x: Length = 4mm;   // explicit type declaration
y = 4mm;           // implicit type declaration via units.
```

Declarations without any initializations are *not allowed* in µcad.
Hence, the following example will fail:

[![test](.test/types_no_declaration.svg)](.test/types_no_declaration.log)

```µcad,types_no_declaration#fail
x: Length;         // error
```

However, for parameter lists in functions and workbenches, you can declare the type only but also pass a default value:

[![test](.test/types_bundles_functions.svg)](.test/types_bundles_functions.log)

```µcad,types_bundles_functions
fn f(x = 4mm) {}        // use unit (with default)
fn f(x: Length) {}     // use type (without default)
```
