# Namespaces

* Provides function and modules
* No parameter list

[![test](.test/namespaces_example.png)](.test/namespaces_example.log)

```µcad,namespaces_example
// namespace module std
namespace std {
    
    // namespace module math
    namespace math {

        // define PI as field
        pi = 3.14159;

        // define calculation function
        function abs(x: Scalar) -> Scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// call both
x = std::math::abs(-1) * std::math::pi;
```
