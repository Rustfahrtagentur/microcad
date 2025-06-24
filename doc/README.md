# µcad language

- [Program Structure](#program-structure)
- [Data Types](#data-types)
- [Calls](#calls)
- [Objects](#objects)
- [Libraries](#libraries)

The *µcad programming language* is purely declarative, which means that a µcad program can be
evaluated like a mathematical equation, resulting in a graphical output.
It only needs to know the values of all the variables to obtain this result.

## Program Structure

A **µcad program** can simply consist of a sequence of [*statements*](structure/statements.md)  in a
*source file* or more complex constructs such as [*workbenches*](structure/workbench.md) and
[*functions*](structure/functions.md).

Additionally, [*modules*](structure/modules.md) help bundle things into packages and resolve naming
collision issues.

With [*use statements*](structure/use.md) functionalities of a module can be used in other modules.

## Data Types

µcad knows several [*primitive types*](types/primitive_types.md) (like `String` and `Integer`)
and [*quantity types*](types/quantity.md) which are always linked to a *unit* (like `Length` in
`mm` or an `Angle` in `°`).

[*Collections*](types/collections.md) (like [*tuples*](types/tuples.md) or [*arrays*](types/arrays.md))
can bundle other types into structured sets.

Produced 3D and 2D objects are stored in [*nodes*](types/nodes.md).

It's planned to implement [*custom types*](types/custom_types.md) in future.

## Calls

In µcad you may call [*workbenches*](structure/calls.md#calling-workbenches) to produce *objects* from [sketches](structure/sketch.md)
or [*parts*](structure/part.md) or you may call [*functions*](structure/calls.md#calling-functions).

In both cases you shall read about [*argument multiplicity*](structure/arguments.md#argument-multiplicity) and
[*argument matching*](structure/arguments.md#argument-matching) to understand how µcad is processing arguments.

## Objects

The 2D or 3D objects produced with [*workbenches*](structure/calls.md#calling-workbenches) can be
[measured](nodes/measures.md) or [exported](attributes/export.md).

## Libraries

µcad knows two kinds of libraries: Those which are written µcad language and those which
are written in *Rust*.

The [standard library](libs/std/README.md) is a µcad library which encapsulates the
[builtin library](libs/builtin.md) into a nice and convenient interface.

If you want to make your own libraries put you µcad code into the search paths or create
a [plugin](libs/plugins.md) to embed Rust code.

## Debugging

There are several builtin [debug functionalities](debug/README.md) which help you debugging your code.
