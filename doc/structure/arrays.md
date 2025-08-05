
# Arrays

An array is an ordered collection of values.

## Arrays as list: `[1, 2, 3]`

[![test](.test/arrays_and_comments.png)](.test/arrays_and_comments.log)

```µcad,arrays_and_comments
[
    // First element
    1,

    // Second element
    2
];
```

You can count the number of elements in an array using `std::count`:

[![test](.test/array_expressions.png)](.test/array_expressions.log)

```µcad,array_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```

## Arrays as range: `[1..3]`

You can generate an array via range expressions: `[1..3]`.

[![test](.test/range_expressions.png)](.test/range_expressions.log)

```µcad,range_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```
