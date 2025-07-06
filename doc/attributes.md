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
This results in two ways to attach an attribute:

* *Name-value pairs*: `#[color = "#FF00FF"]`, `#[resolution = 200%]`. Store and retrieve arbitrary values.

* *Calls*: `#[export("test.svg")]`, `#[svg("style = fill:none")]`. Store export-specific values.


## Color attribute

The `color` attribute attaches a color to a node.

In viewer and when exported, the node will be drawn in the specified color.

```µcad,attributes_color
#[color = "#FFFFFF"]
std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#color, "#FFFFFF");
```

## Resolution attribute

The `resolution` attribute sets the rendering resolution of this node.
The node will be rendered in with 200% resolution than the default resolution of `0.1mm`.
This means the circle will be rendered with a resolution `0.05mm`.

```µcad,attributes_precision
#[resolution = 200%]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#resolution, 200%);
std::debug::assert_eq(c#resolution, 0.05mm);
```

## Exporter specific attributes

Exporter specific attributes have a call-like syntax.

### Export attribute

The `export` defines the filename and the (optional) ID. 
If you have created a part or a sketch and want to export it to a specific file, you can add the export attribute:

```µcad,attributes_export_example
#[export("circle.svg")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#export.filename, "circle.svg");
```

Additional, you can use the `id` parameter to use a specific exporter.
However, the exporter is detected automatically depending on the file extension.

```µcad,attributes_export_id
#[export("circle.svg", id = "svg")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#export.filename, "circle.svg");
```


See [export](export.md) for more information.


### SVG attribute

The `svg` exporter has these attributes:

* `style: String`: The style attribute attached to SVG tag.

```µcad,attributes_export_example
#[export("circle.svg")]
#[svg(style = "fill: skyblue; stroke: cadetblue; stroke-width: 2;")]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(c#export.filename, "circle.svg");
```


