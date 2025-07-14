# Export nodes

When a µcad file is processed by the interpreter, you can export the resulting nodes in a specific file format.

## Export via CLI

To export a µcad file via the CLI, the most simple form is:

```sh
µcad export myfile.µcad # -> myfile.svg
```

You can also give an optional custom name with a specific format:

```sh
microcad-cli export myfile.µcad custom_name.svg # -> custom_name.svg
```

## Default formats

The output format depends on the kind of node:

* *SVG* is the default format for sketches.
* *STL* is the default format for parts.
* If the content is mixed, there will be an error, unless you mark different nodes with export attributes (see next section).

If you call the command line:

```sh
microcad-cli export my_sketch.µcad # This is a sketch and will output `my_sketch.svg`
microcad-cli export my_part.µcad # This is a part and will output `my_part.stl`
```

## Export specific nodes via attributes

Assuming, you have two sketches and want to export each in a specific file.
You assign an *export attribute* with a filename to each sketch.
If you omit the file extension, the default export format will be picked automatically.

[![test](.test/export_attributes.png)](.test/export_attributes.log)

```µcad,export_attributes
#[export("rect.svg")] // Will be exported to `rect.svg`
std::geo2d::rect(42mm);

#[export("circle.svg")]  // Will be exported to `circle.svg`
std::geo2d::circle(r = 42mm);
```

In the CLI, you can select the node specifically:

```sh
microcad-cli export myfile.µcad --list # List all exports in this file: `rect, circle.svg`.
microcad-cli export myfile.µcad --target rect  # Export rectangle node to `rect.svg`
```

