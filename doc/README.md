# µcad language

- [µcad language](#µcad-language)
  - [Program Structure](#program-structure)
  - [Data Types](#data-types)
  - [Workbenches \& Objects](#workbenches--objects)
  - [Exceptional Features](#exceptional-features)
  - [Standard Library](#standard-library)
  - [Builtin Library](#builtin-library)
  - [Debugging](#debugging)

The *µcad programming language* is purely declarative, which means that a µcad program can be
evaluated like a mathematical equation, resulting in a graphical output.
One only needs to know the values of all the variables to obtain this result.

## Program Structure

A **µcad program** can simply consist of a sequence of **statements** or more complex constructs
such as **workbenches** and **functions**, which may depend on each other through **use statements**.
Additionally, **modules** help bundle things into packages and resolve naming collision issues.

- [Modules](modules.md) for modularization of complex code
- [Workbenches](workbench/README.md) produce or manipulate 2D and 3D objects
- [Materials](materials.md) describe materials of objects
- [Statements](statements.md) actual running code

## Data Types

There are several **primitive types** which are always linked to a **unit** (like `Length` in `mm` or an `Angle` in `°`)
and some which just represent scalar values or counts (like `Scalar` or `Integer`).

**Collection types** (like `Tuple` or `Array`) can bundle other types into structured sets.

- [Primitive Types](primitive_types.md)
- [Tuples](tuple.md)
- [Arrays](arrays.md)

## Workbenches & Objects

Workbenches like `sketch` and `part` produce 2D and 3D objects

## Exceptional Features

The µcad language has some more or less obscure features which replace common
programming concepts like for-loops or goto-jumps.

- [Parameter Multiplicity](parameter_multiplicity.md)
- [Unit Types](unit_types.md)

## Standard Library

A big advantage which µcad can take from it's strict modular concept is that
big parts of functionalities can be written in the µcad language itself.

- Standard Library (`std`)
  - [Mathematical Functions (`math`)](std/math.md)
  - [Geometric Algorithms (`algorithm`)](std/algorithm/README.md)
  - [Export Functions](std/export.md)

## Builtin Library

The *builtin library* rely on a few, very basic builtin functions which are
implemented in *Rust* to be fast and to cover internal complexities.

- Export Statement (__builtin::export)
- Basic 2D Primitives (`__builtin::geo2d`)
- Basic 3D Primitives (`__builtin::geo3d`)

## Debugging

- [Diagnostics](diag/README.md)
- [Verification](verify.md)
