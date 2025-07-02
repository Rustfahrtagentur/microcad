# Attributes

Attributes are syntax elements that can be used to attach exporter-specific data to nodes.

The attributes will not change the node geometry itself, but might change its appearance when if they are used for viewers or exporters.
There can be multiple attributes for a node, but each attribute needs to have a unique ID.

## Simple example

Let's define a node `c`.

When viewed or exported, node `c` will have a red color, because the `color` attribute will be set:

```µcad,attributes_simple_example
#[color = "#FFFFFF"]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#color, "#FFFFFF");
```


## Syntax

Syntactically, an attribute consists of `#` prefix and an item.
An attribute item can be a *tag*, a *name-value* pair or a *call*.
This results in two ways to attach metadata:

* *Name-value pairs*: `#[layer = "custom"]`, `#[precision = 200%]`, `#[color = "#FF00FF"]`. Store and retrieve arbitrary values.

* *Calls*: `#[export("test.svg")]`, `#[svg("style = fill:none")]`. Store export-specific values.

## Export attributes

If you have created a part or a sketch and want to export it to a specific file, you can add the export attribute:

```µcad,attributes_export_example
#[export("circle.svg")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#export.filename, "circle.svg");
```

See [export](export.md) for more information.
