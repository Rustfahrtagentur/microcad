
# Tuple expression

Tuples are lists of items which might be of different types.

![test](.banner/tuple_named_tuple.png)

```µcad,tuple_named_tuple
(width=10cm, depth=10cm, volume=1l);
```

## Tuple as module parameters

![test](.banner/tuple_parameters_A.png)

```µcad,tuple_parameters_A#fail
module box((x,y,z) = 0mm) {}
```

![test](.banner/tuple_parameters_B.png)

```µcad,tuple_parameters_B
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

![test](.banner/tuple_parameters_C.png)

```µcad,tuple_parameters_C#fail
module box(x,y,z = 0mm) {}
```

## Field declaration for a module

![test](.banner/tuple_fields_A.png)

```µcad,tuple_fields_A#fail
(width, height) = (1,2)mm;
```

![test](.banner/tuple_fields_B.png)

```µcad,tuple_fields_B
width = 1.2mm;
height = 2mm;
```

![test](.banner/tuple_fields_C.png)

```µcad,tuple_fields_C#fail
(width, height) = (0mm,0mm);
```

![test](.banner/tuple_fields_D.png)

```µcad,tuple_fields_D
width = (0.0, 0.0)mm;
height = (0.0, 0.0)mm;
```

![test](.banner/tuple_fields_E.png)

```µcad,tuple_fields_E#fail
width, height = 0mm;
```
