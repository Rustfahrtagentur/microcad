# Module initializers with `init`

## Implicit init

A module with arguments has one implicit init:

```µcad,implicit_init
module box(size: length) {
    cube(size);
}
```

## Explicit init

```µcad
module box {
    init(size: length) {
        rectangle(size);
    }
}
```

## Explicit init overloading

```µcad
module box {
    init(size: length) {
        rectangle(size);
    }
    init((width, height): length) {
        rectangle(width, height);
    }
}
```

## Default init

```µcad
module box {
    init(size: length) {
        rectangle(size);
    }
    init((width, height): length) {
        rectangle(width, height);
    }
}
```

## Members with multiple inits

TODO: (Besseres Beispiel)

```µcad
module box {
    y := 0mm; // If this field is missing then, an error "MissingField" is raised 

    init(size: length) {
        x := 10mm;
        ...
    }

    init(width: length, height: length) {
        x := 10mm;
        y := 10mm;
        ...
    }
}
```
