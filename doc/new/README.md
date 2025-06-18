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
- [Workbenches](structure/workbench/README.md) produce or manipulate 2D and 3D objects
- [Statements](structure/statements/README.md) actual running code

## Materials

(not available yet)

## Data Types

There are several **primitive types** which are always linked to a **unit** (like `Length` in `mm` or an `Angle` in `°`)
and some which just represent scalar values or counts (like `Scalar` or `Integer`).

**Collection types** (like `Tuple` or `Array`) can bundle other types into structured sets.

- [Primitive Types](data_types/primitive_types.md)
- [Collections](data_types/collections.md)
- [Quantity Types](data_types/quantity.md)
- [Nodes Type](data_types/nodes.md)
- [Custom Types](data_types/custom_types.md)

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
