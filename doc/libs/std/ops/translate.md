# Translate `std::ops::translation`

We can translate objects in 2D and 3D using `std::ops::transate`:

[![test](.test/translate_2d.svg)](.test/translate_2d.log)

```µcad,translate_2d
std::geo2d::Rect(30mm).std::ops::translate(x = 10mm, y = 20mm);
```
