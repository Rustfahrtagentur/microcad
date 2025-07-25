# Verification

µcad provides several builtin functions that help you to avoid bad input parameters.

## Assert

Assertions define constrains of parameters or cases and they bring any rendering to fail immediately.

one form of assertion is a function which gets an expression.
If the expression computes to `false` a compile error will occur at
that point.

[![test](.test/verify_assert.png)](.test/verify_assert.md)

```µcad,verify_assert
std::debug::assert(true, "You won't see this message");
```

[![test](.test/verify_assert_fail.png)](.test/verify_assert_fail.md)

```µcad,verify_assert_fail#fail
std::debug::assert(false, "this assertion fails");
```

## Error

[![test](.test/verify_error.png)](.test/verify_error.md)

```µcad,verify_error#fail
std::error("this should not have happened");
```

## Todo

`todo()` is like `error()` but aims at reminding you to finish code later.

[![test](.test/verify_todo.png)](.test/verify_todo.md)

```µcad,verify_todo
a = 0;

if a == 0 {
    std::info("a is zero");
} else {
    std::todo("print proper message");
}
```
