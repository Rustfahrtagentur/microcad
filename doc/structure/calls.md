# Calls

## Calling Functions

A call of a function consists of just the *identifier* and an [argument list](arguments.md).
and the result is a *value*:

[![test](.test/call_function.png)](.test/call_function.log)

```µcad,call_function
// function definition
fn square(x: Scalar) { x * x }

// call function square with parameter 2 and store result in s
s = square(x = 2)

// check value
std::assert_eq!( s, 4 );
```

## Calling Workbenches

[Workbenches](workbench.md) can be called in the same way as functions
except that the result is a object node.

[![test](.test/call_workbench.png)](.test/call_workbench.log)

```µcad,call_workbench
// function definition
sketch square(size: Length) { 
    std::geo2d::rect(size);
}

// call square with a size and store object node in s
s = square(size=2cm);

// translate object s
std::translate(x = 1cm) s;
```

## Calling Operations

[Operations](op.md) are called differently:

[![test](.test/call_op.png)](.test/call_op.log)

```µcad,call_op
// function definition
fn square(x: Scalar) { x * x }

// call function square with parameter 2
square(x = 2);
```
