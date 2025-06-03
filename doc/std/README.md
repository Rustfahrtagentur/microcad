# The µcad Standard Library `std`

To understand what exactly the *µcad standard library* is doing and what not one needs to understand its relation to the *µcad builtin library (`__builtin`):

1) The *builtin library* covers all **complicated tasks** (like drawing things and complexer calculations) and all communication with the compiler while the *evaluation*.
2) The *standard library* **covers the builtin functionality** and adds more suffisticated interfaces to the core functionalities of the builtin library.

So generically spoken you will find all functionalities of `__builtin` within `std` but in a handier form.

## Namespaces

The main namespace of the µcad standard library `std` is into these top namespaces which group different kinds of functionalities together:

- [`geo2d`](geo2d/README.md): 2D parts (e.g. `circle`, `rect`)
- [`geo3d`](geo3d/README.md): 3D parts (e.g. `sphere`, `cube`)
- [`algorithm`](algorithm/README.md): Algorithms to manipulate 2D and 3D parts (e.g. `translate`, `difference`)
- [`math`](math/README.md): Mathematical solutions (e.g. `abs`, `pi`)

## Functions

### `std::print()`

Print is a *alias* to `__builtin::print()` which is printing stuff to the output console to be read by the user.
