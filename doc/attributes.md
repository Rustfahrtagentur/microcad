# Attributes

Attributes are syntax elements that are used to attach *metadata* to nodes.
The attributes will not change the node geometry itself, but might change its appearance when if they are used for viewers or exporters.
There can be multiple attributes for a node.

For example:

```µcad,metadata_simple_example
use std::debug::*;

#[color = (r = 100%, b = 0%, g = 0%, a = 100%)]
#[layer = "right_side"]
c = std::geo2d::circle(42.0mm);

std::debug::assert_eq(std::debug::type_of(c.meta.color), "Color");
std::debug::assert_eq(std::debug::type_of(c.meta.layer), "String");

std::debug::assert_eq(c.meta.color, (r = 100%, b = 0%, g = 0%, a = 100%));
std::debug::assert_eq(c.meta.layer, "right_side");
```

When viewed or exported, node `c` will have a red color.

The metadata `color` can be accessed by using the `meta` property.

## Syntax

Syntactically, an attribute consists of `#` prefix and an item.
An attribute item can be a *tag*, a *name-value* pair or a *call*.
This results in two ways to attach metadata:

* *Name-value pairs*: `#[layer = "custom"]`, `#[precision = 200%]`, `#[color = rgb()]`. Store and retrieve arbitrary values.

* *Calls*: `#[export("test.svg")`, `#[svg(style = "fill: skyblue;")]`. Store export-specific values.

