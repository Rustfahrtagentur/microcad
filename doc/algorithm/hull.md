# Hull

In the following examples the convex hull of circles is calculated.

```µcad,module.single#todo
hull()
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
```

```µcad,module.multiple#todo
hull() {
    union() {
        translate(x = [-10, 10]mm, y = [-10, 10]mm)
            circle(1mm);
        translate(x = [-20, 20]mm, y = 0mm)
            circle(1mm);
    }
}
```
