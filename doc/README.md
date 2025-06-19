# µcad language

- [µcad language](#µcad-language)
  - [Program Structure](#program-structure)
  - [Materials](#materials)
  - [Data Types](#data-types)
  - [Calls](#calls)
  - [Nodes](#nodes)
  - [Attributes](#attributes)
  - [Libraries](#libraries)
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

- [Modules](structure/modules.md) for modularization of complex code
- [Workbenches](structure/workbench/README.md) produce or manipulate 2D and 3D objects
- [Statements](structure/statements.md) actual running code

## Materials

(not available yet)

## Data Types

There are several **primitive types** which are always linked to a **unit** (like `Length` in `mm` or an `Angle` in `°`)
and some which just represent scalar values or counts (like `Scalar` or `Integer`).

**Collection types** (like `Tuple` or `Array`) can bundle other types into structured sets.

- [Primitive Types](data_types/primitive_types.md)
- [Collections](data_types/collections.md)
- [Tuples](data_types/tuple.md)
- [Arrays](data_types/arrays.md)

## Calls

- [Calling Workbenches](calls/workbench_call.md)
- [Calling Functions](calls/function_call.md)
- [Call Parameters](calls/parameters.md)

## Nodes

## Attributes

- [Export Attributes](attributes/export.md)

## Libraries

- [Standard Library `std`](libs/std.md) 
- [Builtin Library `__builtin`](libs/builtin.md)
- [Plugin Libraries](libs/plugins.md)








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
