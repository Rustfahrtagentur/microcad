# Module Initializers

- [Module Initializers](#module-initializers)
- [Example](#example)
- [Implicit Initializer](#implicit-initializer)
- [Explicit Initializer](#explicit-initializer)
  - [Explicit Initializer overloading](#explicit-initializer-overloading)
- [Calling Module Initializers](#calling-module-initializers)
  - [Call Implicit Initializer](#call-implicit-initializer)
  - [Call Explicit Initializer](#call-explicit-initializer)
  - [Call Implicit, Explicit Init \& Pre-Initialization Code](#call-implicit-explicit-init--pre-initialization-code)

## Example

![test](.banner/init.png)

```µcad,init
// begin module and declare implicit initializer
module donut(radius_outer: Length, radius_inner: Length) {

    // alternative initialization with diameters
    init( diameter_outer: Length, diameter_inner: Length ) {
        // calculate radiuses from diameters
        radius_inner = diameter_inner / 2.0;
        radius_outer = diameter_outer / 2.0;
    }

    // generate donut based on radiuses
    std::geo2d::circle(r = radius_outer) - std::geo2d::circle(r = radius_inner);
}

// generate three equal donuts
donut( 2.0cm, 1.0cm );
donut( radius_outer = 2.0cm, radius_inner = 1.0cm );
donut( diameter_outer = 4.0cm, diameter_inner = 2.0cm );
```

## Implicit Initializer

A module with an *implicit initializer* which takes a `size: Length`:

![test](.banner/init_implicit.png)

```µcad,init_implicit
module box(size: Length) {
    rectangle(size);
}
```

## Explicit Initializer

A module with an *explicit initializer* which takes a `size: Length`:

![test](.banner/init_explicit.png)

```µcad,init_explicit
module double_box {
    init( half_the_size: Length) { size = half_the_size * 2; }
    rectangle(size);
}
```

![test](.banner/init_explicit_overloading.png)

### Explicit Initializer overloading

A module with *multiple explicit initializers* which takes different
parameters:

```µcad,init_explicit_overloading
module box {
    init(size: Length) {
        rectangle(size);
    }

    init(width: Length, height: Length) {
        rectangle(width, height);
    }
}
```

## Calling Module Initializers

### Call Implicit Initializer

Calling an explicit initializer of a module.

![test](.banner/init_call_implicit.png)

```µcad,init_call_implicit
// module with implicit initializer
module m(l: Length) {
    // explicit initializer
    init(f: Length) { r = f/2.0; }
}

// call implicit initializer
m(l=1mm);
```

### Call Explicit Initializer

Calling an explicit initializer of a module.

![test](.banner/init_call_explicit.png)

```µcad,init_call_explicit
// module with implicit initializer
module m(l: Length) {
    // explicit initializer
    init(f: Length) { l = f/2.0; }
    std::geo2d::circle( r = l );
}

// call explicit initializer
m(f=1mm);
```

### Call Implicit, Explicit Init & Pre-Initialization Code

A module with both, *implicit and *explicit initializer* which takes different
parameters and some *pre-initialization code*:

![test](.banner/init_call_implicit_explicit.png)

```µcad,init_call_implicit_explicit
// module with implicit initializer
module m( area: (width: Length, height: Length) ) {
  // pre-initialization code
  default_width = 2.0m;

  // explicit initializer
  init( height: Length) { area = ( width = default_width, height); }
}

// call implicit initializer
m( area = (width = 1.0m, height = 0.5m) );

// use explicit initializer
m( height = 0.5m );
```
