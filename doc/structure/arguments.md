# Arguments

- [Argument Matching](#argument-matching)
  - [Match by Name](#match-by-name)
  - [Match by Type](#match-by-type)
  - [Mix Name and Type Matching](#mix-name-and-type-matching)
  - [Named Tuple Argument Matching](#named-tuple-argument-matching)
  - [Matching Errors](#matching-errors)
- [Argument Multiplicity](#argument-multiplicity)
- [Inline Identifiers](#inline-identifiers)

## Argument Matching

To match *arguments* with *function* or *workbench parameters*, µcad employs
a process known as *argument matching*.

### Match by Name

The following example demonstrates a call to function `f` with each argument
specified by name:

[![test](.test/argument_match_name.svg)](.test/argument_match_name.log)

```µcad,argument_match_name
fn f( x: Length, y: Length, z: Length ) {}

f(x = 1cm, y = 2cm, z = 3cm);
```

### Match by Type

It is possible to use nameless values if all the *parameter types* of a called
function (or workbench) differ in their types.

[![test](.test/argument_match_type.svg)](.test/argument_match_type.log)

```µcad,argument_match_type
fn f( a: Scalar, b: Length, c: Area ) {}
// Who needs names?
f(1.0, 2cm, 3cm²);
```

### Mix Name and Type Matching

You can mix both methods if some parameters cannot be identified by type alone.

[![test](.test/argument_match_mix.svg)](.test/argument_match_mix.log)

```µcad,argument_match_mix
fn f( a: Scalar, b: Length, c: Length ) {}
// `a` is the only Scalar and `b` is named, so `c` does not need a name.
f(1.0, b=2cm, 3cm);
```

### Named Tuple Argument Matching

The concept behind named tuple argument matching is to allow functions to accept
subsets of parameters in a bundle.
This makes it easy to pre-configure parts of arguments:

[![test](.test/tuple_match.svg)](.test/tuple_match.log)

```µcad,tuple_match#todo
// Function with three parameters: x, y, and z
fn f( x: Length, y: Length, z: Length ) {}

// Since we do not want to change x and y in the following statements,
// we prepare a tuple named plane:
plane = (x=1cm, y=2cm);

// Then we pass plane to f() three times with different z values
f( plane, z=3cm);
f( plane, z=6cm);
f( plane, z=9cm);
```

The same function can be called in various ways using named tuples:

[![test](.test/tuple_match_variants.svg)](.test/tuple_match_variants.log)

```µcad,tuple_match_variants#todo
fn f( x: Length, y: Length, z: Length ) {}

// Every parameter given by name
f( x=1cm, y=2cm, z=3cm);

// Parameters given by named tuple
f( (x=1cm, y=2cm, z=3cm) );

// Parameters given by named tuple variable
p = (x=1cm, y=2cm, z=3cm);
f( p );

// Parameters given by named tuple and a single value
f( (x=1cm, y=2cm), z=3cm );
f( y=2cm, (x=1cm, z=3cm) );

// Parameters given by named tuple variable and a single value
q = (x=1cm, y=2cm);
f( q, z=3cm );
```

As you can see, the possibilities are endless.

### Matching Errors

If you do not provide all parameters, you will get an error:

[![test](.test/tuple_match_errors.svg)](.test/tuple_match_errors.log)

```µcad,tuple_match_errors#fail
fn f( x: Length, y: Length, z: Length ) {}
f( (x=1cm, v=2cm), z=3cm); // error: y is missing here
```

When you provide all parameters but some are redundant, you will get a warning:

[![test](.test/tuple_match_warnings.svg)](.test/tuple_match_warnings.log)

```µcad,tuple_match_warnings#fail
fn f( x: Length, y: Length, z: Length ) {}
f( (x=1cm, y=2cm, v=5cm), z=3cm);  // warning: v is redundant
```

## Argument Multiplicity

When working with multiplicities, each argument can be provided as an *array* of elements of a parameter's type.
Each list element will then be evaluated for each of the array's values.
This is known as *argument multiplicity*. This way, we can intuitively express a call that is executed for each argument variant.

The following example will produce 4 rectangles at different positions:

[![test](.test/multiplicity_arrays.svg)](.test/multiplicity_arrays.log)

```µcad,multiplicity_arrays
r = std::geo2d::rect(width = 2mm, height = 2mm);

r.std::ops::translate(x = [-4mm, 4mm], y = [-4mm, 4mm]);
```

The example results in the following calls:

[![test](.test/no_multiplicity.svg)](.test/no_multiplicity.log)

```µcad,no_multiplicity
r = std::geo2d::rect(width = 2mm, height = 2mm);

use std::ops::translate;
r.translate(x = -4mm, y = -4mm);
r.translate(x = -4mm, y = 4mm);
r.translate(x = 4mm, y = -4mm);
r.translate(x = 4mm, y = 4mm);
```

Normally, this would require 2 nested *for loops*, which are not available in µcad.

Another example uses an array of tuples and produces the same output:

[![test](.test/multiplicity_tuple_array.svg)](.test/multiplicity_tuple_array.log)

```µcad,multiplicity_tuple_array#todo
r = std::geo2d::rect(width = 2mm, height = 2mm);

r.std::ops::translate([(x=-4mm, y=-4mm), (x=-4mm, y=4mm), (x=4mm, y=-4mm), (x=4mm, y=4mm)]);
```

## Inline Identifiers

Argument names can be skipped if the parameter expression is a single identifier.
Like in the following example, where the variables `width` and `height` have the
exact same name as the parameters of `circle()`.

[![test](.test/inline_identifiers.svg)](.test/inline_identifiers.log)

```µcad,inline_identifiers
width = 2mm;
height = 2mm;
std::geo2d::rect(width, height);
```
