# Visibility

[![test](.test/visibility.svg)](.test/visibility.log)

```µcad,visibility
mod my {
    mod mod_private {
        pub const_public = 1;
        const const_private = 1;
    }
    pub mod mod_public {
        pub const_public = 1;
        const const_private = 1;
    }
}

__builtin::debug::assert_valid(my.mod_public.const_public);
__builtin::debug::assert_invalid(my.mod_public.const_private);
__builtin::debug::assert_invalid(my.mod_private.const_public);
__builtin::debug::assert_invalid(my.mod_private.const_private);
```
