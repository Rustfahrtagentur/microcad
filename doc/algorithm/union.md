# Union

## Union operator

Express union with binary operator `|`:

```µCAD
circle(r = 3.0mm) | rect(3.0mm);
```

## Union module

```µCAD
union() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```
