# Hull

In the following examples the convex hull of circles is calculated.

![test](.banner/hull_single.png)

```µcad,hull_single#todo
hull()
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
```

![test](.banner/hull_multiple.png)

```µcad,hull_multiple#todo
hull() {
    union() {
        translate(x = [-10, 10]mm, y = [-10, 10]mm)
            circle(1mm);
        translate(x = [-20, 20]mm, y = 0mm)
            circle(1mm);
    }
}
```
