# Modules

## File Modules

TODO

## Mod

[![test](.test/mod_example.svg)](.test/mod_example.log)

```µcad,mod_example
mod my {
    mod math {
        // define PI as property
        const PI = 3.14159;

        // define calculation function
        fn abs(x: Scalar) -> Scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// call both
x = my::math::abs(-1.0) * my::math::PI;
```
