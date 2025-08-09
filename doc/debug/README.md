# Verification

µcad provides several builtin functions that help you to avoid bad input parameters.

## Assert

Assertions define constrains of parameters or cases and they bring any rendering to fail immediately.

one form of assertion is a function which gets an expression.
If the expression computes to `false` a compile error will occur at
that point.

[![test](.test/verify_assert.svg)](.test/verify_assert.log)

```µcad,verify_assert
std::debug::assert(true, "You won't see this message");
```

[![test](.test/verify_assert_fail.svg)](.test/verify_assert_fail.log)

```µcad,verify_assert_fail#fail
std::debug::assert(false, "this assertion fails"); // error
```

## Error

[![test](.test/verify_error.svg)](.test/verify_error.log)

```µcad,verify_error#fail
std::log::error("this should not have happened"); // error
```

## Todo

`todo()` is like `error()` but aims at reminding you to finish code later.

[![test](.test/verify_todo.svg)](.test/verify_todo.log)

```µcad,verify_todo
a = 0;

if a == 0 {
    std::log::info("a is zero");
} else {
    std::log::todo("print proper message");
}
```
