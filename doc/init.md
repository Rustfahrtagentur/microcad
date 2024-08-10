# Module initializers with `init`

## Implicit init

A module with arguments has one implicit init:

```µcad,implicit
module box(size: length) {
    rectangle(size);
}
```

## Explicit init

```µcad,explicit
module box {
    init(size: length) {
        rectangle(size);
    }
}
```

## Init overloading

```µcad,overloading
module box {
    init(size: length) {
        rectangle(size);
    }
    init(width: length, height: length) {
        rectangle(width, height);
    }
}
```

## Default init

```µcad,default
module box() {
    init(size: length) {
        rectangle(size);
    }
    init(width: length, height: length) {
        rectangle(width, height);
    }
}
```

## Members with multiple inits

TODO: (Besseres Beispiel)

```µcad,multiple_inits
module box {
    // If this field is missing then, an error "MissingField" is raised 
    y = 0mm;

    init(size: length) {
        x = 10mm;
        // ...
    }

    init(width: length, height: length) {
        x = 10mm;
        y = 10mm;
        // ...
    }
}
```
