# Module Use

```µCAD,use_statement#todo
module donut(outer: length, inner: length) {
    // load circle from module geo2d
    use circle from std::geo2d;

    // circle is now without geo2d prefix
    circle(outer) - circle(inner);
}

donut(2cm,1cm);
```
