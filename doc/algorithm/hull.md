# Hull

## Hull operator `°`

In the following examples the hull of two circles is calculated.

```µCAD,operator
translate(x = [-10, 10]mm, y = [-10, 10]mm)
    circle(1mm);
```

## Hull module

```µCAD,module.single
hull()
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
```

```µCAD,module.multiple
hull() {
    union() {
        translate(x = [-10, 10]mm, y = [-10, 10]mm)
            circle(1mm);
        translate(x = [-20, 20]mm, y = 0mm)
            circle(1mm);
    }
}
```
