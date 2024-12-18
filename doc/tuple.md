
# Tuple expression

Tuples are lists of items which might be of different types.

```µcad,tuples
(width=10cm, depth=10cm, volume=1l);
```

## Tuple as module parameters

```µcad,parameters.A#fail
module box((x,y,z) = 0mm) {}
```

```µcad,parameters.B
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

```µcad,parameters.C#fail
module box(x,y,z = 0mm) {}
```

## Field declaration for a module

```µcad,fields.A#fail
(width, height) = (1,2)mm;
```

```µcad,fields.B
width = 1.2mm;
height = 2mm;
```

```µcad,fields.C#fail
(width, height) = (0mm,0mm);
```

```µcad,fields.D
width = (0.0, 0.0)mm;
height = (0.0, 0.0)mm;
```

```µcad,fields.E#fail
width, height = 0mm;
```
