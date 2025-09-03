# External module

Let's assume we want to use the Lego_brick from an external module - like a library.

Fortunately, this is simple!
We just have to create a second file `my_brick.µcad`:

```µcad
microcad create my_brick
```

The directory structure is supposed to contain these files:

```plain
lego_brick.µcad
my_brick.µcad
```

Let's add the following content to the `my_brick.µcad` file to
create a few bricks with different parameters:

[![test](.test/library.svg)](.test/library.log)

```µcad,library
mod lego_brick;

use lego_brick::*;

// 2x2 double height
double_2x2 = LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2);

// 4x2 single height
single_4x2 = LegoBrick(rows = 4, columns = 2);

// 3x2 one-third height
third_3x2 = LegoBrick(rows = 3, columns = 2, base_height = 3.2mm);

// generate geometry placing all elements side by side
single_4x2;
double_2x2.translate(y = -40mm);
third_3x2.translate(y = 40mm);
```

![Picture](.test/library-out.svg)

We use `mod` to load our external module