# Union

## Union operator

Express union with binary operator `|`:

```µCAD,union.operator
circle(r = 3.0mm) | rect(size = 3.0mm);
```

## Union module

```µCAD,union.module
use * from std;

algorithm::union() {
    geo2d::circle(radius = 3.0mm);
    geo2d::rect(x=0.0, y=0.0, width = 3.0, height = 2.0);
}
```
