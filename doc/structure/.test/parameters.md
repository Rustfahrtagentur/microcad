# Test [`parameters`](/doc/structure/op.md#L53)

## Code

```µcad
// define operation which takes multiple items
op punch_disk(radius: Length) {
    if @children.count() == 1 {
        difference() { 
            @children 
            std::geo2d::circle(radius)
        } 
    } else {
        std::error("punch_disk must get one object");
    }
}

// use operation punch_disk on a circle
punch_disk(radius = 1cm) {
    std::geo2d::circle(radius = 2cm);
}

```

## Parse Error

```,plain
Parser error:  --> 3:8
  |
3 |     if @children.count() == 1 {
  |        ^---
  |
  = expected COMMENT or expression```

Parser error:  --> 3:8
  |
3 |     if @children.count() == 1 {
  |        ^---
  |
  = expected COMMENT or expression
## Test Result

![FAIL (TODO)](/doc/structure/.test/parameters.png)
