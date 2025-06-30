
# Tuples

A *tuple* is a collection of *values*, each of which can be of different *types*.
Typically, each value is paired with an identifier, allowing a tuple to function
similarly to a key-value store.

[![test](.test/named_tuple.png)](.test/named_tuple.log)

```µcad,named_tuple
tuple = (width=10cm, depth=10cm, volume=1l);
```

## Partially Unnamed Tuples

A tuple may lack identifiers for some or even all of its values.
In such cases, these *unnamed values* within the tuple must all be of different types.

[![test](.test/unnamed_tuple.png)](.test/unnamed_tuple.log)

```µcad,unnamed_tuple
tuple = (10cm, 10cm², 1l);
```

Otherwise, they would be indistinguishable since the values in a tuple do not adhere
to a specific order.

[![test](.test/unnamed_tuple_order.png)](.test/unnamed_tuple_order.log)

```µcad,unnamed_tuple_order
// these tuples are equal
std::debug::assert_eq((1l, 10cm, 10cm²), (10cm, 10cm², 1l));
```

A partially or fully unnamed tuple can only be utilized through
[argument matching](../structure/arguments.md#argument-matching) or *tuple assignment*.

## Tuple Assignments

Tuple syntax is also employed on the left side of *tuple assignments*.

[![test](.test/tuple_assignment.png)](.test/tuple_assignment.log)

```µcad,tuple_assignment#todo
(width, height) = (1m,2m);
// check values of width and height
assert_eq(width,1m);
assert_eq(height,2m);
```

Occasionally, it's practical to *group units* together, but this cannot be done directly
with a tuple.
Instead, you must use an *array*, which will be converted into a tuple during the assignment.

[![test](.test/tuple_assignment_bundle.png)](.test/tuple_assignment_bundle.log)

```µcad,tuple_assignment_bundle#todo
(width, height) = [1,2]m;
assert_eq(width,1m);
assert_eq(height,2m);
```

This method can generally be used to convert an *array* into a *tuple*:

[![test](.test/tuple_assignment_convert.png)](.test/tuple_assignment_convert.log)

```µcad,tuple_assignment_convert#todo
array = [1,2]m;
(width, height) = array;
tuple = (width, height);

assert_eq(tuple,(1m,2m));
assert_eq(tuple.width,1m);
assert_eq(tuple.height,2m);
```
