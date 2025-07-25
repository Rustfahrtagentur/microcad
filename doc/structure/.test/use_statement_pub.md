# Test [`use_statement_pub`](/doc/structure/use.md#L103)

## Code

```µcad
mod my {
    pub use std::geo3d::*;
}

my::sphere(r = 4mm);
my::cube(size = 40mm);

```

