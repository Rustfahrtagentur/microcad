
# Tuple expression

Tuples are lists of items which might be of different types.

![test](.test/tuple_named_tuple.png)
[see build log](.test/tuple_named_tuple.log)

```µcad,tuple_named_tuple
(width=10cm, depth=10cm, volume=1l);
```

## Tuple as module parameters

![test](.test/tuple_parameters_A.png)
[see build log](.test/tuple_parameters_A.log)

```µcad,tuple_parameters_A#fail
module box((x,y,z) = 0mm) {}
```

![test](.test/tuple_parameters_B.png)
[see build log](.test/tuple_parameters_B.log)

```µcad,tuple_parameters_B
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

![test](.test/tuple_parameters_C.png)
[see build log](.test/tuple_parameters_C.log)

```µcad,tuple_parameters_C#fail
module box(x,y,z = 0mm) {}
```

## Field declaration for a module

![test](.test/tuple_fields_A.png)
[see build log](.test/tuple_fields_A.log)

```µcad,tuple_fields_A#fail
(width, height) = (1,2)mm;
```

![test](.test/tuple_fields_B.png)
[see build log](.test/tuple_fields_B.log)

```µcad,tuple_fields_B
width = 1.2mm;
height = 2mm;
```

![test](.test/tuple_fields_C.png)
[see build log](.test/tuple_fields_C.log)

```µcad,tuple_fields_C#fail
(width, height) = (0mm,0mm);
```

![test](.test/tuple_fields_D.png)
[see build log](.test/tuple_fields_D.log)

```µcad,tuple_fields_D
width = (0, 0)mm;
height = (0, 0)mm;
```

![test](.test/tuple_fields_E.png)
[see build log](.test/tuple_fields_E.log)

```µcad,tuple_fields_E#fail
width, height = 0mm;
```
