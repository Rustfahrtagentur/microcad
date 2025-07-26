
# Arrays

An array is an ordered collection of values.

```µcad,arrays_and_comments
[
    // First element
    1,

    // Second element
    2
];
```

You can count the number of elements in an array using `std::count`:

```µcad,array_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```
