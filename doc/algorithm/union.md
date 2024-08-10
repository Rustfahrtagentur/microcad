# Union

## Union operator

Express union with binary operator `|`:

```µCAD,union.operator
circle(r = 3.0mm) | rect(size = 3.0mm);
```

## Union module

```µCAD,union.module
union() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```
