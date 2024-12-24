# Types

## Declaration

In µcad you will need to use units almost everywhere you use values.
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

This one is just needed, if you declare parameters of a function or a module without giving any default value:

[![test](.test/types_bundles_functions.png)](.test/types_bundles_functions.log)

```µcad,types_bundles_functions
function f( x = 4mm ) {}        // use unit (with default)
function f( x : Length ) {}     // use type (without default)
```
