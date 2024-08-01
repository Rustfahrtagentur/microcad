
### Difference

```µCAD
module donut(r_outer: length, r_inner: length) {
    circle(r_outer) - circle(r_inner);
}
```

```µCAD
use algorithm::*;

module donut(r_outer: length, r_inner: length) {
    difference([circle(r_outer), circle(r_inner)]) {
    
    }

}
```

module difference() {

}

# Algorithmic operators

## Boolean operators

### Union

#### `|` operator

Express union with binary operator `|`:

```µcad
circle(r = 3.0mm) | rect(3.0mm);
```

#### Use `union` as a module with parameters

```µcad
union([circle(r = 3.0mm), rect(3.0mm)]);
```

Modules can be passed parameter lists.

#### With `{}` nested modules

```µcad
union() {
    circle(r = 3.0mm);
    rect(size = 3.0mm);
}
```

### Intersection

## Hull

```µcad


```
