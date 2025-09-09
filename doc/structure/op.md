# Operations

*Operations* process 2D xor 3D geometries into 2D xor 3D geometries.

Actual operations are workbenches that process or transform their child *object
nodes* to generate a new geometry.

So this would be a neutral operation:

[![test](.test/op_example.svg)](.test/op_example.log)

```µcad,op_example
// define operation nop without parameters
op nop() { @input }

// use operation nop on a circle
std::geo2d::Circle(radius = 1cm).nop();
```

## `@input`

`@input` is a placeholder to tell where child nodes are nested.
It can also be used to retrieve information about the tree structure
In the above example `@input` will result in a `std::geo2d::Circle(radius = 1cm)`.

An operation can have multiple children like in this example:

[![test](.test/input.svg)](.test/input.log)

```µcad,input#todo
// define operation which takes multiple items
op punched_disk() { 
    // check number of children
    if @input.count() == 2 {
        // make hole
        @input.subtract(); 
    } else {
        std::error("punched_disk must get two objects");
    }
}

// use operation punch_disk with two circles
{
    std::geo2d::Circle(radius = 1cm);
    std::geo2d::Circle(radius = 2cm);
}.punched_disk();
```

Like other workbenches operations can have parameters too:

[![test](.test/parameters.svg)](.test/parameters.log)

```µcad,parameters#todo
// define operation which takes multiple items
op punch_disk(radius: Length) {
    if @input.count() == 1 {
        { 
            @input;
            std::geo2d::Circle(radius);
        }.subtract();
    } else {
        std::error("punch_disk must get one object");
    }
}

// use operation punch_disk on a circle
{
    std::geo2d::Circle(radius = 2cm);
}.punch_disk(radius = 1cm);
```
