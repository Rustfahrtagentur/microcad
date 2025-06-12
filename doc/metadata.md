# Node metadata

You can attach *metadata* to nodes using a special syntax.
Metadata will not change the node geometry itself, but might change its appearance when if they are used for viewers or exporters.
There can be multiple meta data items.

For example:

```µcad,metadata_simple_example
#[color = "red", aux]
#[layer = "right_side"]
c = std::geo2d::circle(42.0mm);

assert(c.meta.color == "red");
```

When viewed or exported, node `c` will have a red color.

The metadata `color` can be accessed by using the `meta` property.

## Syntax

Syntactically, a meta item consists of `#` prefix and an item.
An attribute item can be a *tag*, a *name-value* pair or a *call*.
This results in four ways to attach metadata:

* *file imports*: `#[import("data.toml")]`: Loads `metadata.toml` and evaluates it.

* *tags*: `#[aux]`. Simple tags to store and retrieve boolean values.

* *name-value pairs*: `#[layer = "custom"]`, `#[precision = 200%]`, `#[color = rgb()]`. Store and retrieve arbitrary values.

* *calls*: `#[export("test.svg")`, `#[svg(style = "fill: skyblue;")]`. Store export specific values.



### Tag metadata

Tags assign a boolean value with tag name to node.
The following example will assign a tag with name `aux` to a circle:

```µcad,metadata_tag
#[aux]
circle_a = std::geo2d::circle(r = 42.0mm);
std::debug::assert(circle_a.meta.aux);

circle_b = std::geo2d::circle(r = 23.0mm);
std::debug::assert(!circle_b.meta.aux);
```

Model nodes marked with `aux` tag will be visible in the viewer but will not be exported.
Currently, only `aux` is available as built-in tag attribute.

### Name-value metadata

Name-value attributes attach an object node with metadata as a key value pair.
These name value pairs can be considered as special variables.


```µcad,attributes_name_value
// Assign the sphere to `layer_a` and assign blue as color 
#[layer = "layer_a"]
#[color = "blue"]
std::geo3d::sphere(r = 4.0mm);

```

The following built-in name-value attributes are available:

#### `layer`

Assign the object and its children to a layer. 
This is useful to distinguish nodes by layer.

```µcad,metadata_layer
#[layer = "bearing"]
std::geo3d::sphere(r = 4.0mm);
```

#### `color`

Assign a color to the object, displayed in the viewer.

Assign green color to a cylinder:

```µcad,metadata_color
#[color = "green"]
std::geo3d::cylinder(r = 4.0mm);
```

#### `resolution`

Control the resolution of the object when it is processed.

This cylinder will be rendered with 200% precision.

```µcad,metadata_precision
#[resolution = 200%]
std::geo3d::cylinder(r = 4.0mm);

#[resolution.angle = 10°]
std::geo3d::cylinder(r = 4.0mm);

#[resolution.length = 1mm]
std::geo3d::cylinder(r = 4.0mm);
```

You can also control the number vertices and segments directly:

```µcad,metadata_precision:
#[resolution.vertices = 32]
std::geo3d::cylinder(r = 4.0mm);


#[resolution.segments = 4]
std::geo3d::cylinder(r = 4.0mm);
```



### Exporter specific metadata

Metadata  are mostly metadata that is tight specifically to an exporter. 

#### `export`: Controlling the way a file is exported.

An `export` item instructs the interpreter to export this node to the file `cube.stl`.

```µcad,metadata_call
#[export("cube.stl")]
std::geo3d::cube(4.0mm);
```

This is short for:

```µcad,metadata_export_call
#[export(filename = "cube.stl", format = "ascii")]
std::geo3d::cube(4.0mm);
```


```µcad,meta
#[export(filename = "<id>.stl", format = "ascii")]
test = cube(42mm);
```

This will export a cube in `test.stl`.

```
#[export(import("metadata.toml", key = "<workbench>"))]
part my_cube() {
  std::geo3d::cube(42mm);  
}

```

The TOML file looks like this:

```toml
[my_part]
filename = "<id>.stl"
format = "ascii"
```





#### `import`: Loading metadata from a file.


```µcad
#[import("metadata.toml")]
```

The 

#### `svg`: Scalable vector graphics

Assume, you want to draw a circle with a certain style when it is exported to an SVG.
Use the `svg` attribute and its `style` parameter to annotate the model node `c`:

```µcad,attributes_svg_example
#[svg(style = "fill: skyblue; stroke: cadetblue; stroke-width: 2;")]
c = std::geo2d::circle(42.0mm);
```

The SVG exporter will use string in `style` and will add it as an attribute to the resulting SVG element.
The exported SVG result will look like this:

```svg
<circle cx="0" cy="0" r="42" style="fill: skyblue; stroke: cadetblue; stroke-width: 2;"/>
```


#### `bom`: Bill of materials

A you can attach metadata like a part id to a model node:

```µcad,attributes_part
#[bom(name = "An awesome cube")]
part a_cube() {
  std::geo3d::cube(4.0mm);
}

cube = a_cube();

// Access the part id attribute like a property
std::debug::assert(cube.meta.bom.name == "An awesome cube);
```




In the example above, we attach the part id `42` to the cube's metadata and access its value.
For example, the part id can later used to identify instances of the same object, for example to generate a bill of materials.


```µcad,metadata_bom
MY_ID = "ID";

#[bom(id = "{MY_ID}{@instance_id}")]
part my_part() { 
  std::geo2d::rect(2mm);
}

std::debug::assert(my_part().meta.bom.id == "ID1");
std::debug::assert(my_part().meta.bom.id == "ID2");
```


Currently, only `export` is available as built-in call attribute.


## Metadata contexts

A declaration of metadata are only allowed in certain contexts. 
Declaring metadata in unsupported contexts is not valid.

The following statements can have metadata:

* *expression statement*: Add metadata the the expression. Only valid if the expression evaluates into a *node*.

  This circle will be red:

  ```µcad,attributes_expression
  #[color = "red"]
  std::geo2d::circle(2mm);
  ```

* *assignment statement*: Add metadata to the assignment. Only valid of the result of the assignment is a *node*.

  The variable `rect` will contain a blue rectangle:

  ```µcad,attributes_assignment
  #[color = "blue"] 
  rect = std::geo2d::rect(2mm);

  // Assign color of `rect` to `circle`
  #[color = rect.meta.color]
  circle = std::geo2d::circle(2mm);
  ```

* *part definition*: Add metadata to all nodes created by this part definition.

  For example, all instances created by this part have the ID `MYPART`:
  

### Variable capture

Attributes can capture variables that are part of the current scope:

```µcad,metadata_capture
id = 32;
#[layer = id]
rect = std::geo2d::rect(3.0mm);

std::debug::assert(rect.meta.layer == id);
```

The means you can parametrize metadata in parts:

```µcad,metadata_part_export
part export(filename: String) {
    #[export(filename)] {
        @children
    }
}
```
