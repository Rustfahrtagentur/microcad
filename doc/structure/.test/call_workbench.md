# Test [`call_workbench`](/doc/structure/calls.md#L28)

## Code

```µcad
// definition of a sketch workbench
sketch square(size: Length) { 
    std::geo2d::rect(size);
}

// call square with a size and store object node in s
s = square(size=2cm);

// translate object s
std::ops::translate(x = 1cm) s;

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/call_workbench.png)
