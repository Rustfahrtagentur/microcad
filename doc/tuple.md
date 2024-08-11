
# Tuple expression

## Tuple as module parameters

```µCAD,parameters.A
module box((x,y,z) = 0mm) {}
```

```µCAD,parameters.B
module box(x = 0mm, y = 0mm, z = 0mm) {}
```

```µCAD,parameters.C
module box(x,y,z = 0mm) {}
```

## Field declaration for a module

```µCAD,fields.A#fail
(width, height) = (1,2)mm;
```

```µCAD,fields.B
width = 1.2mm;
height = 2mm;
```

```µCAD,fields.C#fail
(width, height) = (0mm,0mm);
```

```µCAD,fields.D
width = (0.0, 0.0)mm;
height = (0.0, 0.0)mm;
```

```µCAD,fields.E#fail
width, height = 0mm;
```
