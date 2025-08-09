# Union

## Union operator

Express union with binary operator `|`:

[![test](.test/union_operator.svg)](.test/union_operator.log)

```µcad,union_operator
std::geo2d::circle(radius = 3mm) | std::geo2d::rect(width = 3mm, height = 2mm);
```

## Alternative union operator

[![test](.test/union_alt_operator.svg)](.test/union_alt_operator.log)

```µcad,union_alt_operator
{
    std::geo2d::circle(radius = 3mm);
    std::geo2d::rect(width = 3mm, height = 2mm);
}.std::ops::union();
```
