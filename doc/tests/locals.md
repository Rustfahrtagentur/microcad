# Locals and scope tests

## Locals

[![test](.test/locals.svg)](.test/locals.log)

```µcad,locals
// This tests the local stack

use __builtin::debug::*;

// new local variable #1
i = 1;

{
    // accessing #1
    assert(i == 1);

    // new local variable #2 with same name
    i = 2;

    // accessing #2
    assert(i == 2);
};

// accessing #1
assert(i == 1);

// concentric scopes
{
    {
        {
            // accessing #1
            assert(i == 1);
        }
    }
};

// overwrite #1
i = 3;

// access #1
assert(i == 3);

p = __builtin::math::PI;

q = p > 3.;

assert(q);
```

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
