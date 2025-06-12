# µcad language

- [µcad language](#µcad-language)
  - [Program Structure](#program-structure)
  - [Data Types](#data-types)
  - [Obscure Features](#obscure-features)
  - [Builtin Libraries](#builtin-libraries)
  - [Builtin Functions](#builtin-functions)
  - [Debugging](#debugging)

The *µcad* programming language is purely declarative so a *µcad* program can
be calculated like a mathematical equation.
One just need to know all values of all the variables.

## Program Structure

A **µcad program** can be just a sequence of instructions or a more complex construct of
separate **parts** and **functions** which may depend on each other via **use statements**.
**Modules** help to bundle things into packages and solve naming collisions.

- [Modules](modules.md) bundle together parts, (sub)modules or use statements
- [Workbenches](workbench/README.md) produce or manipulate 2D and 3D objects
- [Materials](materials.md) describe materials of objects
- [Statements](statements.md) actual running code

## Data Types

There are several **primitive types** which are always linked to a unit (like `Length` in `mm` or an `Angle` in `°`)
and some which just represent factors or counts (like `Scalar` and `Integer`).

**Collection types** (like `Tuple` or `Array`) can bundle other types into structured parameter sets.

- [Primitive Types](primitive_types.md)
- [Tuples](tuple.md)
- [Arrays](arrays.md)

## Obscure Features

The µcad language has some more or less obscure features which replace common
programming concepts like for-loops or goto-jumps.

- [Parameter Multiplicity](parameter_multiplicity.md)
- [Unit Types](unit_types.md)

## Builtin Libraries

A big advantage which µcad can take from it's strict modular concept is that
big parts of the basic functionalities can be written in the µcad language itself.

- Standard Library (`std`)
  - [Mathematical Functions (`math`)](std/math.md)
  - [Geometric Algorithms (`algorithm`)](std/algorithm/README.md)
  - [Export Functions](std/export.md)

## Builtin Functions

The *builtin libraries* rely on a few, very basic builtin functions which are
implemented in *Rust* to be fast and to cover internal complexities.

- Export Statement (__builtin::export)
- Basic 2D Primitives (`__builtin::geo2d`)
- Basic 3D Primitives (`__builtin::geo3d`)

## Debugging

- [Diagnostics](diag/README.md)
- [Verification](verify.md)
