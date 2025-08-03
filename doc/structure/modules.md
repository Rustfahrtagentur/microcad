# Modules

## File Modules

TODO

## Mod

[![test](.test/mod_example.png)](.test/mod_example.log)

```µcad,mod_example
mod std {
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
x = std::math::abs(-1) * std::math::PI;
```
