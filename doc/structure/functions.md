# **Functions**

- [Declaration](#declaration)
- [Module Functions](#module-functions)
- [Workbench Functions](#workbench-functions)
  - [Restrictions](#restrictions)

*Functions* provide a way to encapsulate frequently used code into sub-routines.
These sub-routines can then be [called](calls.md) to execute their code with a
specific set of [parameters](parameters.md).

[![test](.test/example.png)](.test/example.log)

```µcad,example
// define function print_error with text as parameter of type String
fn print_error( text: String ) {
    // code body
    std::print("ERROR: {text}");
}

print_error("first");
print_error("second");
```

Functions may be declared within [source files](source_files.md), [modules](modules.md) or [workbenches](workbench.md).

## Declaration

A *function declaration* starts with the keyword `fn`, followed by an *identifier*,
a *parameter list*, and a *function body*.
Functions can also return a value as *result*:

[![test](.test/return.png)](.test/return.log)

```µcad,return
fn pow( x: Scalar, n: Integer ) {
    if n == 1 {
        x   // return x
    } else {
        x * pow(n-1) // return recursive product
    }
}
```

Returning a value twice is not allowed.

[![test](.test/return_twice.png)](.test/return_twice.log)

```µcad,return_twice
fn pow( x: Scalar, n: Integer ) {
    if n == 1 {
        x 
    }
    x * pow(n-1)  // error: unexpected code
}
```

## Module Functions

A [module](modules.md) can contain functions that are accessible within the module.
By declaring a function as *public* using the keyword `pub`, it becomes available for
use outside the module.

[![test](.test/mod.png)](.test/mod.log)

```µcad,mod#todo_fail
// module math
mod math {
    // pow cannot be called from outside math
    fn pow( x: Scalar, n: Integer ) {
        if n == 1 {
            x   // return x
        } else {
            x * pow(n-1) // return recursive product
        }
    }

    // square is callable from outside math
    pub fn square(x: Scalar) {
        // call internal pow
        pow(x,2)
    }
}

// call square in math
math::square(2);
math::pow(2,5);  // error: pow is private
```

## Workbench Functions

A [workbench](workbench.md) can contain functions that are accessible within the module only.

Here is an example which generates a punched disk of a given radius using a function `inner()`:

[![test](.test/workbench_example.png)](.test/workbench_example.log)

```µcad,workbench_example
part punched_disk(radius: Length) {
    use std::geo2d::circle;

    // calculate inner from radius in a method
    fn inner() { radius/2 }

    // generate donut (and call inner)
    circle(radius) - circle(inner());
}

punched_disk(radius = 1cm);
```

### Restrictions

There are some restrictions for *workbench functions*:

Trying to make them public with the keyword `pub` will result into an error:

[![test](.test/workbench_pub.png)](.test/workbench_pub.log)

```µcad,workbench_pub#todo_fail
part punched_disk(radius: Length) {
    pub fn inner() { radius/2 }   // error: cant use pub inside workbench
}
```

You cannot create *workbench properties* within the code body.

[![test](.test/workbench_fn_prop.png)](.test/workbench_fn_prop.log)

```µcad,workbench_fn_prop#fail
part punched_disk(radius: Length) {
    fn inner() { 
        prop hole = radius/2;  // eval error: prop not allowed in function
    }
}

punched_disk(1cm);
```
