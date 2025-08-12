# Use Statements

- [Use Statement](#use-statement)
- [Use As Statement](#use-as-statement)
- [Use All Statement](#use-all-statement)
- [Public Use Statement](#public-use-statement)
- [Tests](#tests)

When including code from other *modules* or [other files](modules.md#file-modules)
*fully qualified names* of *symbols* (e.g. `std:geo3d::cube`) often produce much
boiler plate code when using them often.
The powerful `use` statement solves this problem and gives you an elegant method
to import code from other sources.

Internally every *use statement* builds one or more *aliases*, each with an
*identifier* and a *target symbol* it points to.

The following example which uses two *parts* of `geo3d` shows the problem:

[![test](.test/none.svg)](.test/none.log)

```µcad,none
std::geo3d::Sphere(radius = 40mm);
std::geo3d::Cube(size = 40mm);
```

## Use Statement

With `use` it first seems not shorter, but if we would use `sphere` and `cube` more often this would
shorten things a lot:

[![test](.test/use.svg)](.test/use.log)

```µcad,use
use std::geo2d::Circle;
use std::geo2d::Rect;

Circle(r = 4mm);
Rect(size = 40mm);
```

You may also use whole the *module* if the names you are using already exist as a symbol:

[![test](.test/use_module.svg)](.test/use_module.log)

```µcad,use_module
circle = 1;

use std::geo2d;

geo2d::Circle(r = 40mm);
```

## Use As Statement

Another way to be explicit when name conflicts exist is to use `use as` where you can
locally rename the *target symbol*:

[![test](.test/use_as.svg)](.test/use_as.log)

```µcad,use_as
circle = 1;

use std::geo2d::Circle as disk;

disk(r = 4mm);
```

Or you may use `use as` with a *module*:

[![test](.test/use_as_module.svg)](.test/use_as_module.log)

```µcad,use_as_module
geo2d = 1;

use std::geo2d as geo;

geo::Circle(r = 4mm);
```

## Use All Statement

The shortest way to use many symbols from one module is to put an `*` at the end.
The following example aliases **all** symbols of `std::geo3d` into the current scope.

[![test](.test/use_all.svg)](.test/use_all.log)

```µcad,use_all
use std::geo3d::*;

Sphere(r = 4mm);
Cube(size = 40mm);
```

## Public Use Statement

This statement does not only make the *target symbol* visible on the current scope but in
the symbol table where outside code might use it too.

`Sphere` and `Cube` will be made available for using them outside of module `my` in the following example:

[![test](.test/use_statement_pub.svg)](.test/use_statement_pub.log)

```µcad,use_statement_pub
mod my {
    pub use std::geo2d::*;
}

my::Circle(r = 4mm);
my::Rect(size = 40mm);
```

## Tests

[![test](.test/use_statement_pub_in_module.svg)](.test/use_statement_pub_in_module.log)

```µcad,use_statement_pub_in_module
mod my {
    mod name {
        mod space {
            pub use std::geo2d::*;
        }
    }
    pub use name::space::*;
}

my::Circle(r = 4mm);
my::Rect(size = 40mm);
```
