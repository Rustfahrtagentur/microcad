# Modules

[![test](.test/module_example.png)](.test/module_example.log)

```µcad,module_example
mod std {
    mod math {
        // define PI as property
        pi = 3.14159;

        // define calculation function
        fn abs(x: Scalar) -> Scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// call both
x = std::math::abs(-1) * std::math::pi;
```
