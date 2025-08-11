# Arrays

An array is an ordered collection of values with a common type.

## Arrays as list: `[1, 2, 3]`

[![test](.test/arrays_and_comments.svg)](.test/arrays_and_comments.log)

```µcad,arrays_and_comments
[
    // First element
    1,

    // Second element
    2
];
```

You can count the number of elements in an array using `std::count`:

[![test](.test/array_expressions.svg)](.test/array_expressions.log)

```µcad,array_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```

## Arrays as range: `[1..3]`

You can generate an array via range expressions: `[1..3]`.

[![test](.test/range_expressions.svg)](.test/range_expressions.log)

```µcad,range_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```

## Array operations

[![test](.test/array_operations.svg)](.test/array_operations.log)

### Unit bundling

Array support unit bundling, which means the you can write the unit after the `[]` brackets.

[![test](.test/array_unit_bundling.svg)](.test/array_unit_bundling.log)

```µcad,array_unit_bundling
// without bundling
l1 = [1mm, 2mm, 3mm];

// with bundling
l2 = [1, 2, 3]mm;

// are the same
std::debug::assert(l1 == l2);
```

### Addition `*`

#### Adding a quantity

```µcad,array_add
d = 0.5mm;
std::debug::assert_eq([[1,2]mm + 2mm, [3,4]mm]);
```

### Subtraction `-`

#### Subtracting a quantity

```µcad,array_sub
d = 0.5mm;
std::debug::assert_eq([[1,2]mm - 2mm, [-1,0]mm]);
```

### Multiplication `*`

#### Scaling an array

```µcad,array_mul_scale
d = 0.5mm;
std::debug::assert_eq([[-d,d]*2, [-1,1]mm]);
```

### Division `/`

#### Dividing an array by a value

```µcad,array_div
d = 1.0mm;
std::debug::assert_eq([[-d,d]/2, [-0.5, 0.5]mm]);
```
