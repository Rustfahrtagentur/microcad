# Usage of semicolon with Workbenches

[![test](.test/operation_with_body.svg)](.test/operation_with_body.log)

```µcad,operation_with_body
use std::geo2d::circle;
use std::ops::translate;

{ // op with body
    circle(radius = 5mm);
}.translate(y=[-34mm/2 , 34mm/2]);
```

[![test](.test/operation_no_body.svg)](.test/operation_no_body.log)

```µcad,operation_no_body
use std::geo2d::circle;
use std::ops::translate;

 // op without body
circle( radius = 5mm )
    .translate(y = [-34mm/2 , 34mm/2]);
```

[![test](.test/sketch_missing_semicolon.svg)](.test/sketch_missing_semicolon.log)

```µcad,sketch_missing_semicolon#fail
use std::geo2d::circle;
use std::ops::translate;

{
    circle(radius = 5mm) // error: missing semicolon
}.translate(y=[-34mm/2 , 34mm/2]);
```

[![test](.test/sketch_with_empty_body.svg)](.test/sketch_with_empty_body.log)

```µcad,sketch_with_empty_body#fail
use std::geo2d::circle;

{}.circle(radius = 5mm) // error: sketch with body
```

[![test](.test/sketch_with_body.svg)](.test/sketch_with_body.log)

```µcad,sketch_with_body#fail
use std::geo2d::circle;

circle(radius = 2mm) { circle(radius = 1mm); } // error: sketch with body
```

[![test](.test/empty_op.svg)](.test/empty_op.log)

```µcad,empty_op#fail
std::ops::translate(x = 3.0mm); // Error: Translate no geometry. 
{}.std::ops::translate(x = 3.0mm);  // Error: Translate empty geometry.
```

[![test](.test/group.svg)](.test/group.log)

```µcad,group
use std::geo2d::circle;
use std::ops::translate;

// group
{ 
    circle(radius = 1mm); 
    circle(radius = 2mm); 
}
```

[![test](.test/group_assignment.svg)](.test/group_assignment.log)

```µcad,group_assignment
use std::geo2d::circle;
use std::ops::translate;

// assignment + group
a = { 
    circle(radius = 1mm); 
    circle(radius = 2mm); 
};
```
