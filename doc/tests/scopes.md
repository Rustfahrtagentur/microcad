# Scope tests

## Scopes

[![test](.test/scopes.svg)](.test/scopes.log)

```µcad,scopes
a = 1;

__builtin::debug::assert_valid(a);
__builtin::debug::assert_invalid(b);
__builtin::debug::assert_invalid(c);

{
    __builtin::debug::assert_valid(a);
    __builtin::debug::assert_invalid(b);
    __builtin::debug::assert_invalid(c);

    b = 2;

    __builtin::debug::assert_valid(a);
    __builtin::debug::assert_valid(b);
    __builtin::debug::assert_invalid(c);

    c = 3;

    __builtin::debug::assert_valid(a);
    __builtin::debug::assert_valid(b);
    __builtin::debug::assert_valid(c);
};

__builtin::debug::assert_valid(a);
__builtin::debug::assert_invalid(b);
__builtin::debug::assert_invalid(c);
```
