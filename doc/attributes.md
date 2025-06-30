# Attributes

Attributes are syntax elements that can be used to attach exporter-specific data to nodes.
Assuming, you have two sketches and want to export each in a specific file.
You assign an *export attribute* with a filename to each sketch:

```
#[export("rect.svg")]
std::geo2d::rect(42mm);

#[export("circle.svg")]
std::geo2d::circle(r = 42mm);
```

The attributes will not change the node geometry itself, but might change its appearance when if they are used for viewers or exporters.
There can be multiple attributes for a node.

When you use the CLI to export, you can write:

```sh
µcad export myfile.µcad # -> myfile.svg
µcad export myfile.µcad --list # List all exports in this file
µcad export myfile.µcad rect.svg  # Export to `rect.svg`
µcad export myfile.µcad --all # Export all exports in this file: `rect.svg, circle.svg`
```

## Quick examples

### Set Color 

### Set Layer

```sh
µcad export myfile.µcad --layer "top" rect.svg  # Export to `rect.svg`

```

### Set Precision

### Mark as export





```µcad,metadata_simple_example
use std::debug::*;

#[color = "#FFFFFF"]
#[layer = "right_side"]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#color, "#FFFFFF");
std::debug::assert_eq(c#layer, "right_side");
```

When viewed or exported, node `c` will have a red color.

The metadata `color` can be accessed by using the `meta` property.

## Syntax

Syntactically, an attribute consists of `#` prefix and an item.
An attribute item can be a *tag*, a *name-value* pair or a *call*.
This results in two ways to attach metadata:

* *Name-value pairs*: `#[layer = "custom"]`, `#[precision = 200%]`, `#[color = rgb()]`. Store and retrieve arbitrary values.

* *Calls*: `#[export("test.svg")`. Store export-specific values.

## Export attributes

If you have created a part or a sketch and want to export it to a specific file, you can add the export attribute:

```
#[export("test.svg")]
circle(r = 42);
```

Let's annotate these nodes to be exported:


