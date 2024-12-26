
# Record Expressions

Records are ordered lists of items which might be of different types and can have names.
You can say they are a combination of *structs* and *records* like known from other languages.

[![test](.test/record_named_record.png)](.test/record_named_record.log)

```µcad,record_named_record
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

[![test](.test/record_parameters_A.png)](.test/record_parameters_A.log)

```µcad,record_parameters_A#fail
module box((x,y,z) = 0mm) {}
```

[![test](.test/record_parameters_C.png)](.test/record_parameters_C.log)

```µcad,record_parameters_C#fail
module box(x,y,z = 0mm) {}
```

## Record as module parameters

[![test](.test/record_fields_A.png)](.test/record_fields_A.log)

```µcad,record_fields_A#fail
(width, height) = (1,2)mm;
```

[![test](.test/record_fields_B.png)](.test/record_fields_B.log)

```µcad,record_fields_B
width = 1.2mm;
height = 2mm;
```

[![test](.test/record_fields_C.png)](.test/record_fields_C.log)

```µcad,record_fields_C#fail
(width, height) = (0mm,0mm);
```

[![test](.test/record_fields_D.png)](.test/record_fields_D.log)

```µcad,record_fields_D
width = (0, 0)mm;
height = (0, 0)mm;
```

[![test](.test/record_fields_E.png)](.test/record_fields_E.log)

```µcad,record_fields_E#fail
width, height = 0mm;
```

## Matching Records

Records can be matched against *parameter definitions* if the record includes exactly all necessary items the parameter definition is asking for (expect *default parameters* may be missing).

[![test](.test/record_matching.png)](.test/record_matching.log)

```µcad,record_matching
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

Examples for records not matching the parameter list are:

[![test](.test/record_matching_err1.png)](.test/record_matching_err1.log)

```µcad,record_matching_err1#fail
function rectangle( x: Scalar, y: Scalar, w: Scalar, h: Scalar) {}
rectangle( (a=0, x=1, y=2), (w=3, h=4));
```

[![test](.test/record_matching_err2.png)](.test/record_matching_err2.log)

```µcad,record_matching_err2#fail
function rectangle( x: Scalar, y: Scalar, w: Scalar, h: Scalar) {}
rectangle( (x=1, y=2), (x=2, w=3, h=4));
```

[![test](.test/record_matching_err3.png)](.test/record_matching_err3.log)

```µcad,record_matching_err3#fail
pos = (a=3, b=4);
size = (w=3, h=4);
rectangle( pos, size );
```

[![test](.test/record_matching_err4.png)](.test/record_matching_err4.log)

```µcad,record_matching_err4#fail
pos = (3, 4);
size = (w=3, h=4);
rectangle( pos, size );
```

[![test](.test/record_matching_err5.png)](.test/record_matching_err5.log)

```µcad,record_matching_err5#fail
pos1 = (x=3, y=4);
pos2 = (x=3, y=4);
rectangle( (pos1, size, pos2) );
```
