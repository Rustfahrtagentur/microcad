# Control Export

Until now when we export the whole the geometry as an STL this results
in a single output file:

```sh
microcad export lego_brick
```

But we also can use the `#[export]` attribute to export each brick to a different file:

[![test](.test/export.svg)](.test/export.log)

```µcad,export
mod lego_brick;

use lego_brick::*;

#[export = "lego_brick2x2.stl"]
brick2x2 = LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2);

#[export = "lego_brick4x2.stl"]
brick4x2 = LegoBrick(rows = 4, columns = 2);

#[export = "lego-brick5x1.stl"]
brick3x2 = LegoBrick(rows = 5, columns = 1, base_height = 3.2mm);
```

When we export the file now, three files with the specified names will be created
and we do not need the `translate()` operations anymore.

## TODO

- It seems unclear why the export attribute is used at assignments when we
  said before, that assignments will not generate any geometry.
