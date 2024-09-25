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

## Implicit initializers from parameter list (also relates to module parameters)

```µcad,implicit_init_by_parameter.A
function f(a:length, b:vec2) {}

f(a=1cm,b=(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
```

```µcad,implicit_init_by_parameter.B
function f(a:length, b: vec2 = (x=1cm,y=2cm)) {}

f(1cm);
f(1cm,(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
f(a=1cm,b=(x=1cm,y=2cm));
```

```µcad,implicit_init_by_parameter.C
function f(a:length=2cm, b = (x=1cm,y=2cm)) {}

f();
f(1cm);
f(1cm,(x=1cm,y=2cm));
f((x=1cm,y=2cm));
f(a=1cm,b=(x=1cm,y=2cm));
f(1cm,b=(x=1cm,y=2cm));
f(a=1cm,(x=1cm,y=2cm));
```
