# Namespaces

* Provides function and modules
* No parameter list

```µCAD,namespaces
// namespace module std
namespace std {
    
    // namespace module math
    namespace math {

        // define PI as field
        pi = 3.14159;

        // define calculation function
        function abs(x: scalar) -> scalar {
            if x < 0 { -x } else { x }
        }
    }
}

// call both
x = std::math::abs(-1) * std::math::pi;
```
