# Builtin Library `debug`

## Print

[![test](.test/builtin_print.svg)](.test/builtin_print.log)

```µcad,builtin_print
__builtin::print("ok");
```

## Assertions

### `assert`

[![test](.test/builtin_debug_assert.svg)](.test/builtin_debug_assert.log)

```µcad,builtin_debug_assert_true
__builtin::debug::assert(true);
```

[![test](.test/builtin_debug_assert.svg)](.test/builtin_debug_assert.log)

```µcad,builtin_debug_assert_false#fail
__builtin::debug::assert(false); // error: Assertion failed: false
```

### `assert_eq`

### `assert_valid`

### `assert_invalid`
