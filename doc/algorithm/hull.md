# Hull

## Hull operator

In the following examples the hull of two circles is calculated.

```µCAD
°translate(x = [-10, 10]mm, y = [-10, 10]mm)
    circle(1mm);
```

## Hull module

```µCAD
hull()
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
```

```µCAD
hull() {
    translate(x = [-10, 10]mm, y = [-10, 10]mm)
        circle(1mm);
    translate(x = [-20, 20]mm, y = 0mm)
        circle(1mm);
}
```
