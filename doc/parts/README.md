# Parts

- [Parts](#parts)
  - [Declaration](#declaration)
    - [Implicit initialization](#implicit-initialization)
    - [Pre-initialization code](#pre-initialization-code)
    - [Explicit Initializers](#explicit-initializers)
    - [Post-initialization code](#post-initialization-code)
  - [Further information about part elements](#further-information-about-part-elements)
  - [Examples](#examples)

*Parts* are µcad constructs which produce graphical *objects* (2D or 3D) by being initialized with some *parameters* and code that generates the output.

On the first look *parts* in µcad look similar to so-called *classes* in other programming languages
but also they differ quite a bit.

## Declaration

A *part* consist of the following elements:

- A leading keyword `part`,
- an **identifier** that follows and names the *part*,
- an **implicit initialization** via a parameter list attached to the *part identifier*.
- maybe some **pre-initialization code** which is processed after implicit but before explicit initialization,
- maybe some **explicit initializers** which provide alternative initialization methods,
- maybe some **functions** which are sub routines with their own parameters and code bodies,
- maybe some **properties** which may be accessed from the outside and result from *initializers* or *assignment statements* within the code,
- and mostly a **code body** (or *post-initialization code*) which runs after any initialization and produces *objects* as output,

The following (stupid) code demonstrates most of these elements:

[![test](.test/part_declaration.png)](.test/part_declaration.log)

```microcad
part my_part() {}
```

```µcad
part my_part() {}
```

```mcad
part my_part() {}
```

```µcad,part_declaration
// define custom part circle with an implicit initializer
part small_disc(diameter: Length) {

    // pre-initialization code
    radius = diameter /2;  // set property `radius`

    // explicit initializer (overwrites property `radius` by it's parameter)
    init(radius: Length) {
        // must set all properties from implicit initialization parameter list
        diameter = radius * 2.0;
    }

    fn half(value: Length) {
        return value / 2.0;
    }

    use std::geo2d::circle;

    // (post-initialization) code produces a 2D circle with half the diameter given
    circle(half(diameter));
}

use std::debug::assert;

// call part and check property `diameter`
assert(small_disc(diameter = 1cm).diameter == 1cm);
assert(small_disc(radius = 1cm).diameter == 2cm);
```

### Implicit initialization

The *parameter list* of the part definition's header automatically sets all given parameters as properties of the part.
Those may be accessed from within (post-initialization) code body or in functions and from outside.

### Pre-initialization code

Pre-initialization code may just produce new *properties* and can not access the properties which would be generated
by the *implicit initialization*.

### Explicit Initializers

Explicit initializers are always named `init` with a following parameter list.
One may define multiple explicit initializers wich must have different parameters.
From the outside one can not see which initializer type is called.

For each *parameter* of the implicit initialization *explicit initializers* must have either...

- a parameter of the same name
- or set a property with that name.

All `init` methods define the [explicit initializers](init.md).
It's not allowed to write code between them.

### Post-initialization code

Post-initialization code is either the code of a part without any *explicit initializers* or the code after them.
It can access all fields generated from implicit or explicit initialization or from any explicit initializer.

## Further information about part elements

- [Functions](functions.md)
- [Implicit Initialization](parameter_list.md)
- [Explicit Initializers (`init`)](init.md)
- [Properties](property.md)

## Examples

- [Examples](EXAMPLES.md)
