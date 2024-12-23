# Union

## Union operator

Express union with binary operator `|`:

![test](.banner/union_operator.png)

```µcad,union_operator
std::geo2d::circle(radius = 3mm) | std::geo2d::rect(width = 3mm, height = 2mm);
```

## Union module

![test](.banner/union_module.png)

```µcad,union_module
std::algorithm::union() {
    std::geo2d::circle(radius = 3mm);
    std::geo2d::rect(width = 3mm, height = 2mm);
}
```
