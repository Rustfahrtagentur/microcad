# Builtin Library

The *Builtin library* is the bridge between µcad and Rust and covers all tasks that are not supposed to be in the µcad
language itself (like drawing things and more complex calculations). It handles the communications with compiler and
the geometry backend during evaluation.

- 2D geometric primitives: [`geo2d`](geo2d.md)
- 3D geometric primitives: [`geo3d`](geo3d.md)
- geometric operations: [`ops`](ops.md)
- mathematics: [`math`](math.md)
- debugging: [`debug`](debug.md)
