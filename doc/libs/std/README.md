# Standard Library

The *Standard Library* is written in µcad and **covers builtin functionality** and adds more sophisticated interfaces to the core functionalities of the *Builtin Library*.
So generically spoken you will find all functionalities of `__builtin` within `std` but in a handier form.

## Modules

The main module of the µcad standard library `std` consists of these modules which group different kinds of functionalities together:

- [`geo2d`](geo2d/README.md): 2D parts (e.g. `circle`, `rect`)
- [`geo3d`](geo3d/README.md): 3D parts (e.g. `sphere`, `cube`)
- [`ops`](ops/README.md): Algorithms to manipulate 2D and 3D parts (e.g. `translate`, `difference`)
- [`math`](math/README.md): Mathematical solutions (e.g. `abs`, `pi`)
- [`debug`](debug/README.md): Debugging functions

## Functions

### `std::print()`

Print is a *alias* to `__builtin::print()` which is printing stuff to the output console to be read by the user.
