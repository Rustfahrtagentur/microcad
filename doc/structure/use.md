# Use Statements

- [Use Statement](#use-statement)
- [Use As Statement](#use-as-statement)
- [Use All Statement](#use-all-statement)
- [Public Use Statement](#public-use-statement)

When including code from other *modules* or [other files](modules.md#file-modules)
*fully qualified names* of *symbols* (e.g. `std:geo3d::cube`) often produce much
boiler plate code when using them often.
The powerful `use` statement solves this problem and gives you an elegant method
to import code from other sources.

Internally every *use statement* builds one or more *aliases*, each with an
*identifier* and a *target symbol* it points to.

The following example which uses two *parts* of `geo3d` shows the problem:

[![test](.test/none.png)](.test/none.log)

```µcad,none
std::geo3d::sphere(radius = 40mm);
std::geo3d::cube(size = 40mm);
```

## Use Statement

With `use` it first seems not shorter, but if we would use `sphere` and `cube` more often this would
shorten things a lot:

[![test](.test/use.png)](.test/use.log)

```µcad,use
use std::geo2d::circle;
use std::geo2d::rect;

circle(r = 4mm);
rect(size = 40mm);
```

You may also use whole the *module* if the names you are using already exist as a symbol:

[![test](.test/use_module.png)](.test/use_module.log)

```µcad,use_module
circle = 1;

use std::geo2d;

geo2d::circle(r = 40mm);
```

## Use As Statement

Another way to be explicit when name conflicts exist is to use `use as` where you can
locally rename the *target symbol*:

[![test](.test/use_as.png)](.test/use_as.log)

```µcad,use_as
circle = 1;

use std::geo2d::circle as disk;

disk(r = 4mm);
```

Or you may use `use as` with a *module*:

[![test](.test/use_as_module.png)](.test/use_as_module.log)

```µcad,use_as_module
geo2d = 1;

use std::geo2d as geo;

geo::circle(r = 4mm);
```

## Use All Statement

The shortest way to use many symbols from one module is to put an `*` at the end.
The following example aliases **all** symbols of `std::geo3d` into the current scope.

[![test](.test/use_all.png)](.test/use_all.log)

```µcad,use_all#todo
use std::geo3d::*;

sphere(r = 4mm);
cube(size = 40mm);
```

## Public Use Statement

This statement does not only make the *target symbol* visible on the current scope but in
the symbol table where outside code might use it too.

`sphere` and `cube` will be made available for using them outside of module `my` in the following example:

[![test](.test/use_statement_pub.png)](.test/use_statement_pub.log)

```µcad,use_statement_pub
mod my {
    pub use std::geo2d::*;
}

my::circle(r = 4mm);
my::rect(size = 40mm);
```

[![test](.test/use_statement_pub_extra.png)](.test/use_statement_pub_extra.log)

```µcad,use_statement_pub_extra
mod my {
    mod name {
        mod space {
            pub use std::geo2d::*;
        }
    }
    pub use name::space::*;
}

my::circle(r = 4mm);
my::rect(size = 40mm);
```
