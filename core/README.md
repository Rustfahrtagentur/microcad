# µCAD core crate

This crate contains the core functionality of µCAD.
It provides the basic data structures used to handle geometries and operations on them.

## mod `render`

This module contains the rendering logic for the core crate.
It provides the `Render` trait, which is implemented by all types that can be rendered.

This specific renderers like `SvgRenderer` can be found in the `microcad_render` crate.

## mod `geo2d`

Provides the basic 2D geometry types and operations on them.

## mod `export`

Provides the `Exporter` trait, which allows exporting geometries to various formats.
