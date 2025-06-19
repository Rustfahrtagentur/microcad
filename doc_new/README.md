# µcad language

- [Program Structure](#program-structure)
- [Materials](#materials)
- [Data Types](#data-types)
- [Calls](#calls)
- [Nodes](#nodes)
- [Attributes](#attributes)
- [Libraries](#libraries)

The *µcad programming language* is purely declarative, which means that a µcad program can be
evaluated like a mathematical equation, resulting in a graphical output.
One only needs to know the values of all the variables to obtain this result.

## Program Structure

A **µcad program** can simply consist of a sequence of **statements** or more complex constructs
such as **workbenches** and **functions**, which may depend on each other through **use statements**.
Additionally, **modules** help bundle things into packages and resolve naming collision issues.

- [Modularization](structure/modules.md) for modularization of complex code
- [Workbenches](structure/workbench.md) produce or manipulate 2D and 3D objects
- [Statements](structure/statements.md) actual running code

## Materials

(not available yet)

## Data Types

There are several **primitive types** which are always linked to a **unit** (like `Length` in `mm` or an `Angle` in `°`)
and some which just represent scalar values or counts (like `Scalar` or `Integer`).

**Collection types** (like `Tuple` or `Array`) can bundle other types into structured sets.

- [Primitive Types](types/primitive_types.md)
- [Quantity Types](types/quantity.md)
- [Collections](types/collections.md)
- [Nodes Type](types/nodes.md)
- [Custom Types](types/custom_types.md)

## Calls

TODO: Intro text

- [Calling Workbenches](calls/workbench_calls.md)
- [Calling Functions](calls/function_calls.md)
- [Call Parameters](calls/parameters.md)

## Nodes

TODO: Intro text

- [Measures](nodes/measures.md)

## Attributes

TODO: Intro text

- [Export Attributes](attributes/export.md)

## Libraries

TODO: Into text

- [Standard Library `std`](libs/std.md)
- [Builtin Library `__builtin`](libs/builtin.md)
- [Plugin Libraries](libs/plugins.md)
