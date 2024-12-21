# test

![test](.banner/lib_test_fail.png)

```µcad,lib_test_fail
module lib_test(l: length) {
    init(f: length) { r = f/2.0; }
}
lib_test(f=1mm);
```

![test](.banner/lib_test_ok.png)

```µcad,lib_test_ok
module lib_test(l: length) {
    init(f: length) { r = f/2.0; }
}
lib_test(l=1mm);
```

![test](.banner/lib_test1.png)

```µcad,lib_test1
module f( area: area ) {
  default_width = 2.0m;
  init( height: length) { area = ( width = default_width, height); }
}

f( area = (width = 1.0m, height = 0.5m) );
f( height = 0.5m );
```
