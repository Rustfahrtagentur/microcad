# Test [`no_multiplicity`](/doc/structure/arguments.md#L144)

## Code

```µcad
std::ops::translate(x = -4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::ops::translate(x = -4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::ops::translate(x = 4mm, y = -4mm) std::geo2d::rect(width = 2mm, height = 2mm);
std::ops::translate(x = 4mm, y = 4mm) std::geo2d::rect(width = 2mm, height = 2mm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/no_multiplicity.png)
