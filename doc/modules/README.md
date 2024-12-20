# Modules

Modules in µcad are similar to so-called *classes* in other programming languages
but also they differ quite a bit.

A module consist of an *identifier*, some *initializers*, *fields*  and *code* which can be run by calling the module like a function.
If code in a module gets complex it can be separated into functions.

The *parameters* of a module can be defined directly behind the module name (*implicitly*) as well as by several *init()* functions (*explicitly*) which are quite similar to *constructors* in other programming languages.


## Declaration

![test](.banner/modules_declaration.png)

```µcad,modules_declaration#todo
/// use builtin function circle from standard library
use std::geo2d::circle;

// define custom module circle
module small_disc() {
    // generate circle
    circle(1cm);
}

// generate small_disc
small_disc();
```

## Module Elements

* [Functions](functions.md)
* [Explicit Initialization (`init`)](init.md)
* [Implicit Initialization](parameter_list.md)
* [Fields](fields.md)

## Examples

* [Examples](EXAMPLES.md)
