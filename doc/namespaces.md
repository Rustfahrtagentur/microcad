# Namespaces

* Provides function and modules
* No parameter list

![test](.banner/namespaces_example.png)

```µcad,namespaces_example
// namespace module std
namespace std {
    
    // namespace module math
    namespace math {

        // define PI as field
        pi = 3.14159;

        // define calculation function
        function abs(x: scalar) -> scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// call both
x = std::math::abs(-1.0) * std::math::pi;
```
