# Union

## Union operator

Express union with binary operator `|`:

[![test](.test/union_operator.png)](.test/union_operator.log)

```µcad,union_operator
std::geo2d::circle(radius = 3mm) | std::geo2d::rect(width = 3mm, height = 2mm);
```

## Union module

[![test](.test/union_module.png)](.test/union_module.log)

```µcad,union_module
std::algorithm::union() {
    std::geo2d::circle(radius = 3mm);
    std::geo2d::rect(width = 3mm, height = 2mm);
}
```
