# Part Initializers

- [Part Initializers](#part-initializers)
- [Example](#example)
- [Implicit Initializer](#implicit-initializer)
- [Explicit Initializer](#explicit-initializer)
  - [Explicit Initializer overloading](#explicit-initializer-overloading)
- [Calling Part Initializers](#calling-part-initializers)
  - [Call Implicit Initializer](#call-implicit-initializer)
  - [Call Explicit Initializer](#call-explicit-initializer)
  - [Call Implicit, Explicit Init \& Pre-Initialization Code](#call-implicit-explicit-init--pre-initialization-code)

## Example

[![test](.test/init.png)](.test/init.log)

```µcad,init
// begin part and declare implicit initializer
part donut(radius_outer: Length, radius_inner: Length) {

    // alternative initialization with diameters
    init( diameter_outer: Length, diameter_inner: Length ) {
        // calculate radiuses from diameters
        radius_inner = diameter_inner / 2;
        radius_outer = diameter_outer / 2;
    }

    // generate donut based on radiuses
    std::geo2d::circle(r = radius_outer) - std::geo2d::circle(r = radius_inner);
}

// generate three equal donuts
donut( 2cm, 1cm );
donut( radius_outer = 2cm, radius_inner = 1cm );
donut( diameter_outer = 4cm, diameter_inner = 2cm );
```

## Implicit Initializer

A part with an *implicit initializer* which takes a `size: Length`:

[![test](.test/init_implicit.png)](.test/init_implicit.log)

```µcad,init_implicit
part box(size: Length) {
    std::geo2d::rect(width = size, height = size, x = 0mm, y = 0mm);
}

box(size=2cm);
```

## Explicit Initializer

A part with an *explicit initializer* which takes a `size: Length`:

[![test](.test/init_explicit.png)](.test/init_explicit.log)

```µcad,init_explicit
part double_box(size: Length) {
    init(half_the_size: Length) { size = half_the_size * 2; }
    rectangle(size);
}
```

[![test](.test/init_explicit_overloading.png)](.test/init_explicit_overloading.log)

### Explicit Initializer overloading

A part with *multiple explicit initializers* which takes different
parameters:

```µcad,init_explicit_overloading
part box(width: Length, height: Length) {
    init(size: Length) {
        rectangle(size);
    }

    init(width: Length, height: Length) {
        rectangle(width, height);
    }
}
```

## Calling Part Initializers

### Call Implicit Initializer

Calling an explicit initializer of a part.

[![test](.test/init_call_implicit.png)](.test/init_call_implicit.log)

```µcad,init_call_implicit
// part with implicit initializer
part m(l: Length) {
    // explicit initializer
    init(f: Length) { r = f/2; }
}

// call implicit initializer
m(l=1mm);
```

### Call Explicit Initializer

Calling an explicit initializer of a part.

[![test](.test/init_call_explicit.png)](.test/init_call_explicit.log)

```µcad,init_call_explicit
// part with implicit initializer
part m(l: Length) {
    // explicit initializer
    init(f: Length) { l = f/2; }
    std::geo2d::circle( r = l );
}

// call explicit initializer
m(f=1mm);
```

### Call Implicit, Explicit Init & Pre-Initialization Code

A part with both, *implicit and *explicit initializer* which takes different
parameters and some *pre-initialization code*:

[![test](.test/init_call_implicit_explicit.png)](.test/init_call_implicit_explicit.log)

```µcad,init_call_implicit_explicit
// part with implicit initializer
part m(area: (width: Length, height: Length) ) {
  // explicit initializer by height
  init(height: Length) { area = (width = 2m, height = height); }
}

// call implicit initializer
m( area = (width = 1m, height = 0.5m) );

// use explicit initializer
m( height = 0.5m );
```
