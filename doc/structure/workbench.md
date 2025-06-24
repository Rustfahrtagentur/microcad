# Workbenches

- [Workbenches](#workbenches)
  - [Workbench Types](#workbench-types)
  - [Workbench Elements](#workbench-elements)
  - [Workbench Declaration](#workbench-declaration)
    - [Building Plan](#building-plan)
    - [Initializers](#initializers)
    - [Init Code](#init-code)
    - [Building Code](#building-code)
  - [Properties](#properties)

## Workbench Types

*Workbenches* are µcad code constructs used to:

- create 2D [*sketches*](sketch.md) using `sketch`,
- build 3D [*parts*](part.md) using `part`, or
- apply [*operations*](op.md) to them using `op` workbenches.

Essentially, a *workbench* is initialized with a set of *parameters* and possibly some *initialization code*, then executes *code* that generates 2D and 3D *objects*.

## Workbench Declaration

### Workbench Elements

A *workbench* consists of the following elements:

- A leading keyword: `part`, `sketch`, or `op`,
- an **identifier** that names the workbench,
- a **building plan** defined by a *parameter list* following the identifier,
- optional **init code**, which is executed before any *initializer*,
- optional **initializers**, offering ways to initialize the *building plan*,
- optional **functions**, acting as subroutines with their own parameters and code bodies,
- optional **properties**, accessible from outside and defined through initializers or assignments within the code,
- and typically some **building code** (also called *post-initialization code*), which runs after all initialization steps and generates the final *objects*.

The following code demonstrates most of these elements:

[![test](.test/part_declaration.png)](.test/part_declaration.log)

```µcad,part_declaration
// sketch with a `radius` as building plan
sketch wheel(radius: Length) {

    // init code
    const FACTOR = 2;

    // initializer
    init(diameter: Length) {
        // set `radius`
        radius = diameter / FACTOR;
    }

    // function (sub routine)
    fn into_diameter(radius: Length) {
        return radius * FACTOR;
    }

    // building code begins

    // set a property which can be seen from outside
    prop diameter = into_diameter(radius);
    // local variable
    i = 1;
    
    // create circle
    std::geo2d::circle(radius);
}

use std::debug::assert;

// call sketch with diameter
d = wheel(diameter = 2cm)
// check radius
assert(d.radius == 1cm);

// call sketch with radius
r = wheel(radius = 1cm)
// check diameter
assert(r.diameter == 2cm);
```

### Building Plan

The *building plan* is defined by a *parameter list* that follows the workbench's
*identifier*.
All *parameters* in that list become *properties* of the workbench when it is invoked.
These properties can be accessed within the *building code*, inside functions,
or externally.

[![test](.test/building_plan.png)](.test/building_plan.log)

```µcad,building_plan
// sketch with a `radius` as building plan
sketch wheel(radius: Length) {
    // access property `radius` from the building plan
    std::geo2d::circle(radius);
}
__builtin::assert(wheel(5cm).radius == 5cm);
```

### Initializers

*Initializers* are defined with the keyword `init` and a following *parameter list*.
One may define multiple initializers which must have different parameter lists.

[![test](.test/initializers.png)](.test/initializers.log)

```µcad,initializers#fail
part wheel(radius: Length) {
    // error: same parameters as in building plan
    init( radius: Length ) { }
}
```

However, if an initializer is used, all properties from the building plan must
be initialized (except those with *default values*).

[![test](.test/init_property.png)](.test/init_property.log)

```µcad,init_property
part wheel(radius: Length, thickness: Length) {
    // initializer with diameter
    init( diameter: Length, thickness: Length ) {
        // must set `radius` in code 
        radius = diameter / 2;

        // thickness (from the building plan) does not need 
        // to be set, because it was automatically set by 
        // parameter of this initializer
    }

    // Now radius and thickness can be used
    std::geo3d::cylinder(radius, height=thickness);
}

// call with building plan
wheel(radius=5cm, thickness=1cm);
// call with initializer
wheel(diameter=5cm, thickness=1cm);
```

If the *building plan* is not fully initialized by an initializer
you will get an error:

[![test](.test/missed_property.png)](.test/missed_property.log)

```µcad,missed_property#fail
sketch wheel(radius: Length) {
    init( width: Length ) { 
        // misses to set `radius` from building plan
        r = width / 2; 
    }
}
```

### Init Code

If you use *initializers* you might write some *init code*
which must be placed on top of the workbench's body (before any *initializers*).

The *init code* is just allowed to define some *constants* which then can be used
in all following code (including code within *initializers* and *functions*).

[![test](.test/pre_init_code.png)](.test/pre_init_code.log)

```µcad,pre_init_code
sketch wheel(radius: Length) {
    // init code
    const FACTOR = 2.0;

    // function
    fn into_radius( diameter: Length ) {
        // use constant FACTOR from init code
        return diameter / FACTOR;
    }

    // initializer
    init(diameter: Length) { into_radius(radius); }

    // set property diameter and use FACTOR from init code
    prop diameter = radius * FACTOR;
    
    // code body
    std::geo2d::circle(radius);
}
__builtin::assert(wheel(5cm).radius == 5cm);
```

It's **not allowed** to write any code between *initializers*.

[![test](.test/code_between_initializers.png)](.test/code_between_initializers.log)

```µcad,code_between_initializers#fail
sketch wheel(radius: Length) {
    init( width:Length ) { radius = width / 2; }
    
    // error: code between initializers not allowed
    radius = 1;

    init( height:Length ) { radius = height / 2; }
}
```

### Building Code

The *building code* is executed after any initialization.
Usually it produces one or many 2D or 3D objects on base of the given
*building plan*.

[![test](.test/code.png)](.test/code.log)

```µcad,code
sketch wheel(radius: Length) {
    // building code starts here
    std::geo2d::circle(radius);
}
```

If *initializers* were defined the *building code* starts below them.

[![test](.test/code_post_init.png)](.test/code_post_init.log)

```µcad,code_post_init
sketch wheel(radius: Length) {
    // initializer
    init( diameter: Length ) { radius = diameter / 2; }

    // building code starts here
    std::geo2d::circle(radius);
}
```

## Properties

There are two ways to declare *Properties*:

- as parameter of the building plan or
- in an assignment within the build code by using the keyword `prop`.

In the following example we declare a building plan which consists of a `radius` which will automatically be a property:

[![test](.test/property.png)](.test/property.log)

```µcad,property#todo
// `outer` will automatically become a property because
// it is declared in the building plan:
sketch wheel(outer: length) {
    use std::geo2d::circle;

    // `inner` is declared as property and maybe read from 
    // outside this workbench
    prop inner = outer / 2;

    // generate wheel (and use property inner)
    circle(radius) - circle(inner);
}

// evaluate wheel
t = wheel(1cm);

// extract and display `outer` and `inner` from generated wheel
info("outer: {t.outer}");
info("inner: {t.inner}");
```

If you remove the `prop` keyword you will fail at accessing `inner`:

[![test](.test/property_wrong.png)](.test/property_wrong.log)

```µcad,property_wrong#fail
sketch wheel(outer: length) {
    use std::geo2d::circle;

    // `inner` is declared as variable and may not be read
    // from outside this workbench
    inner = outer / 2;

    circle(radius) - circle(inner);
}

t = wheel(outer = 1cm);

// you can still extract and display `outer`
std::info("outer: {t.outer}");
// error: but you cannot access `inner` anymore
std::info("inner: {t.inner}");
```
