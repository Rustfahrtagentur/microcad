# Union

## Union operator

Express union with binary operator `|`:

```µCAD,union.operator
std::geo2d::circle(radius = 3.0) | std::geo2d::rect(x=0.0, y=0.0, width = 3.0, height = 2.0);
```

## Union module

```µCAD,union.module
std::algorithm::union() {
    std::geo2d::circle(radius = 3.0);
    std::geo2d::rect(x=0.0, y=0.0, width = 3.0, height = 2.0);
}
```
