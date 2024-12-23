# Union

## Union operator

Express union with binary operator `|`:

![test](.test/union_operator.png)

```µcad,union_operator
std::geo2d::circle(radius = 3.0mm) | std::geo2d::rect(width = 3.0mm, height = 2.0mm);
```

## Union module

![test](.test/union_module.png)

```µcad,union_module
std::algorithm::union() {
    std::geo2d::circle(radius = 3.0mm);
    std::geo2d::rect(width = 3.0mm, height = 2.0mm);
}
```
