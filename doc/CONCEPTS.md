# Basic Concepts

## The Build Process

The *µcad* interpreter runs programs which generate geometry files.
The processing of *µcad* source code files into output files can be divided into separate phases:

![phases](images/phases.svg)

### Parsing Phase

In the parsing phase the source files are read into a *syntax tree* by using the [*µcad* grammar](lang/grammar.pest).
Any errors which occur within the parsing phase are related to file access or syntax.

### Evaluation Phase

In the evaluation phase the *syntax tree*  will be processed into the *object node tree*
which is a structured representation of the geometry.
While this phase the following things will be done:

- expressions will be calculated
- functions will be called
- modules will generate *object nodes*
- user messages will be output on console

Any errors which occur within the evaluation phase are related to semantic issues.

### Export Phase

In the export phase the *object nodes* will be taken to generate 2D or 3D output files
(e.g. *SVG* or *STL*).
While this phase the following things will be done:

- geometric algorithms will be processed
- geometries will be rendered
- the output files will be written

Any errors which occur within the export phase are related to geometrical processing or file access.

### Viewing

**Note**: Currently *µcad* does not have any available viewer.

The viewing phase generates images which can be shown to visualize *object nodes* (e.g. in an IDE).
Any errors which occur here are related to geometrical processing.
