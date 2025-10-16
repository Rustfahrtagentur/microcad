# Modules

## File Modules

TODO

## Mod

[![test](.test/mod_example.svg)](.test/mod_example.log)

```µcad,mod_example
mod my {
    pub mod math {
        // define PI as property
        pub const PI = 3.14159;

        // define calculation function
        pub fn abs(x: Scalar) -> Scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// call both
x = my::math::abs(-1.0) * my::math::PI;
```
