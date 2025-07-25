# Test [`children`](/doc/structure/op.md#L30)

## Code

```µcad
// define operation which takes multiple items
op punched_disk() { 
    // check number of children
    if @children.count() == 2 {
        // make hole
        difference() { @children } 
    } else {
        std::error("punched_disk must get two objects");
    }
}

// use operation punch_disk with two circles
punched_disk() {
    std::geo2d::circle(radius = 1cm);
    std::geo2d::circle(radius = 2cm);
}

```

## Parse Error

```,plain
Parser error:  --> 4:8
  |
4 |     if @children.count() == 2 {
  |        ^---
  |
  = expected COMMENT or expression```

Parser error:  --> 4:8
  |
4 |     if @children.count() == 2 {
  |        ^---
  |
  = expected COMMENT or expression
## Test Result

![FAIL (TODO)](/doc/structure/.test/children.png)
