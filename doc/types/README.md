# Types

The µcad type system consists of a group built-in types.
The type system is static, which means a declared variable has a fixed type that cannot be changed.

These classes of built-in types are supported:

| Built-in type                     | Description                                                   | Example                                          |
| --------------------------------- | ------------------------------------------------------------- | ------------------------------------------------ |
| [*Quantity*](quantity.md)         | Numeric values with an optional unit.                         | `a: Length = 4mm`                                | 
| [*Bool*](primitive_type.md)       | A boolean value.                                              | `b: Bool = true`                                 |
| [*Integer*](primitive_type.md)    | An integer value without a unit.                              | `c: Integer = 4`                                 |
| [*String*](string.md)             | A string.                                                     | `d: String = "Hello World"`                      |
| [*Array*](array.md)               | A list of values with a *common type*.                        | `e: [Integer] = [1,2,3]`                         |
| [*Tuple*](tuple.md)               | A list of values with a *distinct types*.                     | `f: (Length, Scalar) = (4mm, 4.0)`               |
| [*Named tuple*](named_tuples.md)  | A sorted list of key-value pairs with *distinct types*.       | `g: (x: Scalar, y: Length) = (x = 4.0, y = 4mm)` |
| [*Matrix*](matrix.md)             | Matrix types for affine transforms, for internal usage.       | ---                                              |
| [*Nodes*](nodes.md)               | A node in the model tree.                                     | `h: Node = { cube(2mm); }`                       |   



## Declaration

In µcad, you will need to use units almost everywhere you use values.
This is intended and what you get in return is that declarations are quite handy:

[![test](.test/types_def_vs_decl.png)](.test/types_def_vs_decl.log)

```µcad,types_def_vs_decl
x = 4mm;            // use unit
x : Length = 4mm;   // use type
```

Declarations without any unit is *not allowed* in µcad:

[![test](.test/types_no_declaration.png)](.test/types_no_declaration.log)

```µcad,types_no_declaration#fail
 x: Length;         // error
```

This one is just needed, if you declare parameters giving any default value:

[![test](.test/types_bundles_functions.png)](.test/types_bundles_functions.log)

```µcad,types_bundles_functions
fn f( x = 4mm ) {}        // use unit (with default)
fn f( x : Length ) {}     // use type (without default)
```


## Unit Bundling

Units can be *bundled* in tuples or arrays:

[![test](.test/unit_bundle_tuple.png)](.test/unit_bundle_tuple.log)

```µcad,unit_bundle_tuple
// without unit bundling
p1 = (x=1mm, y=2mm, z=3mm);

// with bundling
p2 = (x=1, y=2, z=3)mm;

// are the same
std::debug::assert(p1 == p2, "Tuples should be equal");
```

[![test](.test/unit_bundle_list.png)](.test/unit_bundle_list.log)

```µcad,unit_bundle_list
// without bundling
l1 = [1mm, 2mm, 3mm];

// with bundling
l2 = [1, 2, 3]mm;

// are the same
std::debug::assert(l1 == l2);
```

