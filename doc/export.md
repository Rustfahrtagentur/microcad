# Export nodes

When a µcad file is processed by the interpreter, you can export the resulting nodes in a specific file format.
For 2D nodes, *SVG* is the default format, where

```sh
microcad-cli export my_sketch.µcad # This is a sketch and will output `my_sketch.svg`
microcad-cli export my_part.µcad # This is a part and will output `my_sketch.stl`
```

## Export via CLI

When you use the CLI to export, you can write:

```sh
µcad export myfile.µcad # -> myfile.svg
µcad export myfile.µcad --list # List all exports in this file
µcad export myfile.µcad rect.svg  # Export to `rect.svg`
µcad export myfile.µcad --all # Export all exports in this file: `rect.svg, circle.svg`
```



## Export via Attributes


```
#[export("rect.svg")]
std::geo2d::rect(42mm);

#[export("circle.svg")]
std::geo2d::circle(r = 42mm);
```
