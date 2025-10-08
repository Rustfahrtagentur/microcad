# Use statement tests

[![test](.test/use_test.svg)](.test/use_test.log)

```µcad,use_test
// use debug from `std/module.µcad`
use std::debug::assert;
assert(true);

// use all symbols from std::debug for test checks
use std::debug::*;

// use symbol `circle` in file `geo2d.µcad`
use std::geo2d::Circle;
assert_valid(Circle);

// use all symbols in file `geo3d.µcad`
use std::geo3d::*;
assert_valid(Sphere);
assert_valid(Cube);

// alias `Circle` in `std/geo2d/mod.µcad` into `foo`
use std::geo2d::Circle as foo;
assert_valid(foo);

// use print from `std/module.µcad`
use std::print;
assert_valid(print);
print("Hello");

// public use operation from `std/module.µcad`
pub use std::ops;
assert_valid(ops);
assert_invalid(use_test::ops);

part MyPart() {
    Circle(radius=1);
    Sphere(radius=1);
}

assert_valid(MyPart);
```
