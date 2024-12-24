
# Tuple Expressions

Tuples are ordered lists of items which might be of different types and can have names.
You can say they are a combination of *structs* and *tuples* like known from other languages.

[![test](.test/tuple_named_tuple.png)](.test/tuple_named_tuple.log)

```µcad,tuple_named_tuple
(width=10cm, depth=10cm, volume=1l);
```

## Tuple as module parameters

[![test](.test/tuple_parameters_A.png)](.test/tuple_parameters_A.log)

```µcad,tuple_parameters_A#fail
module box((x,y,z) = 0mm) {}
```

[![test](.test/tuple_parameters_B.png)](.test/tuple_parameters_B.log)

```µcad,tuple_parameters_B
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

[![test](.test/tuple_parameters_C.png)](.test/tuple_parameters_C.log)

```µcad,tuple_parameters_C#fail
module box(x,y,z = 0mm) {}
```

## Field declaration for a module

[![test](.test/tuple_fields_A.png)](.test/tuple_fields_A.log)

```µcad,tuple_fields_A#fail
(width, height) = (1,2)mm;
```

[![test](.test/tuple_fields_B.png)](.test/tuple_fields_B.log)

```µcad,tuple_fields_B
width = 1.2mm;
height = 2mm;
```

[![test](.test/tuple_fields_C.png)](.test/tuple_fields_C.log)

```µcad,tuple_fields_C#fail
(width, height) = (0mm,0mm);
```

[![test](.test/tuple_fields_D.png)](.test/tuple_fields_D.log)

```µcad,tuple_fields_D
width = (0, 0)mm;
height = (0, 0)mm;
```

[![test](.test/tuple_fields_E.png)](.test/tuple_fields_E.log)

```µcad,tuple_fields_E#fail
width, height = 0mm;
```

## Matching Tuples

Tuples can be matched against *parameter definitions* if the tuple includes exactly all necessary items the parameter definition is asking for (expect *default parameters* may be missing).

[![test](.test/tuple_matching.png)](.test/tuple_matching.log)

```µcad,tuple_matching
function rectangle( x: Scalar, y: Scalar, w: Scalar, h: Scalar) {}

rectangle( x=1, y=2, w=3, h=4);
rectangle( (x=1, y=2), (w=3, h=4));
rectangle( x=1, (y=2, w=3), h=4);
rectangle( (x=1, (y=2, w=3), h=4));

pos = (x=3, y=4);
size = (w=3, h=4);

rectangle( pos, size );
rectangle( x=1, y=1, size );
rectangle( pos, w=1, h=1 );
```

Examples for tuples not matching the parameter list are:

[![test](.test/tuple_matching_err1.png)](.test/tuple_matching_err1.log)

```µcad,tuple_matching_err1#fail
function rectangle( x: Scalar, y: Scalar, w: Scalar, h: Scalar) {}
rectangle( (a=0, x=1, y=2), (w=3, h=4));
```

[![test](.test/tuple_matching_err2.png)](.test/tuple_matching_err2.log)

```µcad,tuple_matching_err2#fail
function rectangle( x: Scalar, y: Scalar, w: Scalar, h: Scalar) {}
rectangle( (x=1, y=2), (x=2, w=3, h=4));
```

[![test](.test/tuple_matching_err3.png)](.test/tuple_matching_err3.log)

```µcad,tuple_matching_err3#fail
pos = (a=3, b=4);
size = (w=3, h=4);
rectangle( pos, size );
```

[![test](.test/tuple_matching_err4.png)](.test/tuple_matching_err4.log)

```µcad,tuple_matching_err4#fail
pos = (3, 4);
size = (w=3, h=4);
rectangle( pos, size );
```

[![test](.test/tuple_matching_err5.png)](.test/tuple_matching_err5.log)

```µcad,tuple_matching_err5#fail
pos1 = (x=3, y=4);
pos2 = (x=3, y=4);
rectangle( (pos1, size, pos2) );
```
