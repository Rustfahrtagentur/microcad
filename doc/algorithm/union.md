# Union

## Union operator

Express union with binary operator `|`:

```µCAD,union.operator
std::geo2d::circle(radius = 3.0mm) | std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Union module

```µCAD,union.module
std::algorithm::union() {
    std::geo2d::circle(radius = 3.0mm);
    std::geo2d::rect(width = 3.0mm, height = 2.0mm);
}
```
