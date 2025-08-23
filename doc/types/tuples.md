
# Tuples

A *tuple* is a collection of *values*, each of which can be of different *types*.
Typically, each value is paired with an identifier, allowing a tuple to function
similarly to a key-value store.

[![test](.test/named_tuple_access.svg)](.test/named_tuple_access.log)

```µcad,named_tuple_access
use std::debug::assert_eq;

tuple = (width=10cm, depth=10cm, volume=1l);

assert_eq([tuple.width, 10cm]);
assert_eq([tuple.depth, 10cm]);
assert_eq([tuple.volume, 1l]);

assert_eq([tuple, (width=10cm, depth=10cm, volume=1l)]);
```

## Partially Unnamed Tuples

A tuple may lack identifiers for some or even all of its values.
In such cases, these *unnamed values* within the tuple must all be of different types.

[![test](.test/unnamed_tuple.svg)](.test/unnamed_tuple.log)

```µcad,unnamed_tuple
tuple = (10cm, 10cm², 1l);
```

Otherwise, they would be indistinguishable since the values in a tuple do not adhere
to a specific order.

[![test](.test/unnamed_tuple_order.svg)](.test/unnamed_tuple_order.log)

```µcad,unnamed_tuple_order
// these tuples are equal
std::debug::assert_eq([(1l, 10cm, 10cm²), (10cm, 10cm², 1l)]);
```

A partially or fully unnamed tuple can only be utilized through
[argument matching](../structure/arguments.md#argument-matching) or *tuple assignment*.

## Tuple Assignments

Tuple syntax is also employed on the left side of *tuple assignments*.

[![test](.test/tuple_assignment.svg)](.test/tuple_assignment.log)

```µcad,tuple_assignment#todo
(width, height) = (1m,2m);
// check values of width and height
assert_eq([width,1m]);
assert_eq([height,2m]);
```

Occasionally, it's practical to *group units* together, but this cannot be done directly
with a tuple.
Instead, you must use an *array*, which will be converted into a tuple during the assignment.

[![test](.test/tuple_assignment_bundle.svg)](.test/tuple_assignment_bundle.log)

```µcad,tuple_assignment_bundle#todo
(width, height) = [1,2]m;
assert_eq([width,1m]);
assert_eq([height,2m]);
```

This method can generally be used to convert an *array* into a *tuple*:

[![test](.test/tuple_assignment_convert.svg)](.test/tuple_assignment_convert.log)

```µcad,tuple_assignment_convert#todo
array = [1,2]m;
(width, height) = array;
tuple = (width, height);

assert_eq([tuple,(1m,2m)]);
assert_eq([tuple.width,1m]);
assert_eq([tuple.height,2m]);
```

## Tuple operators

### Addition `*`

#### Adding two tuples of the same type

[![test](.test/tuple_add_same.svg)](.test/tuple_add_same.log)

```µcad,tuple_add_same
a = (x = 1.0mm, y = 2.0mm);
b = (x = 3.0mm, y = 4.0mm);
std::debug::assert_eq([a + b, (x = 4.0mm, y = 6.0mm)]);
```

[![test](.test/tuple_add_different.svg)](.test/tuple_add_different.log)

```µcad,tuple_add_different#fail
a = (x = 1.0mm, y = 2.0mm);
b = (x = 3.0mm, z = 4.0mm);
c = a + b; // error: Tuple type mismatch for +: lhs=(x: Length, y: Length), rhs=(x: Length, z: Length)
std::debug::assert_eq([c, (x = 4.0mm, y = 6.0mm)]); // error: Array elements have different types: [<INVALID TYPE>, (x: Length, y: Length)]
```

### Subtraction `-`

#### Subtracting a quantity

[![test](.test/tuple_sub.svg)](.test/tuple_sub.log)

```µcad,tuple_sub
a = (x = 2.0mm, y = 3.0mm);
b = (x = 1.0mm, y = 4.0mm);
std::debug::assert_eq([a - b, (x = 1.0mm, y = -1.0mm)]);
```

### Multiplication `*`

#### Scaling a tuple

[![test](.test/tuple_mul_scale.svg)](.test/tuple_mul_scale.log)

```µcad,tuple_mul_scale
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([v*2, (x = 2.0mm, y = 4.0mm)]);
```

### Division `/`

#### Dividing a tuple by a value

[![test](.test/tuple_div.svg)](.test/tuple_div.log)

```µcad,tuple_div
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([v/2, (x = 0.5mm, y = 1.0mm)]);
```

### Negation `-`

[![test](.test/tuple_neg.svg)](.test/tuple_neg.log)

```µcad,tuple_neg
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([-v, (x = -1.0mm, y = -2.0mm)]);
```
