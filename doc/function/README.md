# Functions

* Function signature and return
* Function purity
* immer call by value
* Callables:
  * das kann Module oder Function
  * module(a = 3)
  * d = function(b = 4)
* Function haben eigenen Scope
  * Können nur auf die Parameter zugreifen

## Implicit Initializers from parameter list (also relates to module parameters)

![test](.banner/README_implicit_init_by_parameter_A.png)

```µcad,README_implicit_init_by_parameter_A
function f(a:Length, b:vec2) {}

f(a=1cm,b=(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
```

![test](.banner/README_implicit_init_by_parameter_B.png)

```µcad,README_implicit_init_by_parameter_B
function f(a:Length, b: Vec2 = (x=1cm,y=2cm)) {}

f(1cm);
f(1cm,(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
f(a=1cm,b=(x=1cm,y=2cm));
```

![test](.banner/README_implicit_init_by_parameter_C.png)

```µcad,README_implicit_init_by_parameter_C
function f(a:Length=2cm, b = (x=1cm,y=2cm)) {}

f();
f(1cm);
f(1cm,(x=1cm,y=2cm));
f((x=1cm,y=2cm));
f(a=1cm,b=(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
```
