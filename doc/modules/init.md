# Module Initialization

## Example

![test](.banner/init.png)

```µcad,init
// begin module and declare implicit initializer
module donut(radius_outer: length, radius_inner: length) {

    // alternative initialization with diameters
    init( diameter_outer: length, diameter_inner: length ) {
        // calculate radiuses from diameters
        radius_inner = diameter_inner / 2.0;
        radius_outer = diameter_outer / 2.0;
    }

    // generate donut based on radiuses
    std::geo2d::circle(radius_outer) - std::geo2d::circle(radius_inner);
}

// generate three equal donuts
donut( 2.0cm, 1.0cm );
//donut( radius_outer = 2.0cm, radius_inner = 1.0cm );
//donut( diameter_outer = 4.0cm, diameter_inner = 2.0cm );
```

## Implicit init

A module with arguments has one implicit init:

![test](.banner/init_implicit.png)

```µcad,init_implicit
module box(size: length) {
    cube(size);
}
```

## Explicit init

![test](.banner/init_explicit.png)

```µcad,init_explicit
module box {
    init(size: length) {
        rectangle(size);
    }
}
```

![test](.banner/init_explicit_overloading.png)

## Explicit init overloading

```µcad,init_explicit_overloading
module box {
    init(size: length) {
        rectangle(size);
    }
    init(width: length, height: length) {
        rectangle(width, height);
    }
}
```

## Members with multiple inits

TODO: (Besseres Beispiel)

![test](.banner/init_bad_example.png)

```µcad,init_bad_example#fail
module box {
    y := 0mm; // If this field is missing then, an error "MissingField" is raised 

    init(size: length) {
        x := 10mm;
        ...
    }

    init(width: length, height: length) {
        x := 10mm;
        y := 10mm;
        ...
    }
}
```
