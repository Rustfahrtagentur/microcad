# Verification

µcad provides several builtin functions that help you to avoid bad input parameters.

## Assert

Assertions define constrains of parameters or cases and they bring any rendering to fail immediately.

one form of assertion is a function which gets an expression.
If the expression computes to `false` a compile error will occur at
that point.

```µcad,assert
std::assert(true, "You won't see this message");
```

```µcad,assert_fail#fail
std::assert(false, "this assertion fails");
```

## Panic

```µcad,panic#todo
a = 0;

if a != 0
    std::panic("this should not have happened");
```

## Todo

`todo()` is like `panic()` but aims on reminding you to finish code later.

```µcad,todo#todo
a = 0;

if a = 0 
    std::info("a is zero");
else
    std::todo("print proper message");
```
