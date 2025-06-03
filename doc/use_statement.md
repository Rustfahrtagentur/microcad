# Use statement

*Fully qualified names* of *symbols* (e.g. `std:geo3d::cube`) often produce much boiler plate code
when using them often.
But there is a powerful `use` statement in µcad to solve this problem.

Generally `use` can be used to make long names shorter.

Internally every *use statement* builds one or more *aliases*, each with an *identifier* and a *target symbol* it
points to.

## No use statement

The following example which uses two *parts* of `geo3d` shows the problem:

[![test](.test/no_use_statement.png)](.test/no_use_statement.log)

```µcad,no_use_statement
std::geo3d::sphere(radius = 40mm);
std::geo3d::cube(size = 40mm);
```

## Simple `use` statement

With `use` it first seems not shorter, but if we would use `sphere` and `cube` more often this would
shorten things a lot:

[![test](.test/use_statement.png)](.test/use_statement.log)

```µcad,use_statement
use std::geo3d::sphere;
use std::geo3d::cube;

sphere(r = 4mm);
cube(size = 40mm);
```

## `use` a namespace to be implicit

You may also use whole *namespaces* if the names you are using already exist as a symbol:

[![test](.test/use_statement_namespace.png)](.test/use_statement_namespace.log)

```µcad,use_statement_namespace
sphere = 1;

use std::geo3d;

geo3d::sphere(r = 40mm);
```

## `use as` statement

Another way to be explicit when name conflicts exist is to use `use as` where you can
locally rename the *target symbol*:

[![test](.test/use_statement_as.png)](.test/use_statement_as.log)

```µcad,use_statement_as
sphere = 1;

use std::geo3d::sphere as ball;

ball(r = 4mm);
```

Or you may use `use as` with a *namespace*:

[![test](.test/use_statement_as_namespace.png)](.test/use_statement_as_namespace.log)

```µcad,use_statement_as_namespace
sphere = 1;

use std::geo3d as geo;

geo::sphere(r = 4mm);
```

## `use *` statement

The shortest way to use many symbols from one namespace is to put an `*` at the end.
The following example aliases **all** symbols of `std::geo3d` into the current scope.

[![test](.test/use_statement_all.png)](.test/use_statement_all.log)

```µcad,use_statement_all
use std::geo3d::*;

sphere(r = 4mm);
cube(size = 40mm);
```

## `pub use` statement

This statement does not only make the *target symbol* visible on the current scope but in
the symbol table where outside code might use it too.

`sphere` and `cube` will be made available for using them outside of namespace `my` in the following example:

[![test](.test/use_statement_pub.png)](.test/use_statement_pub.log)

```µcad,use_statement_pub
namespace my {
    pub use std::geo3d::*;
}

my::sphere(r = 4mm);
my::cube(size = 40mm);
```
