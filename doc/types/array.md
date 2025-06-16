# Arrays

Arrays are lists of values with a common type.

## Unit bundling

Array support unit bundling, which means the you can write the unit after the `[]` brackets.

[![test](.test/array_unit_bundling.png)](.test/array_unit_bundling.log)

```µcad,array_unit_bundling
// without bundling
l1 = [1mm, 2mm, 3mm];

// with bundling
l2 = [1, 2, 3]mm;

// are the same
std::debug::assert(l1 == l2);
```

