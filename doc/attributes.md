# Attributes

Attributes are built-in syntax elements to enrich statements with additional information or act as instructions to the µcad interpreter.
For example, you can attach metadata like a part id to an object node:

```µcad,attributes_part_id
#[part_id = 42]
cube = std::geo3d::cube(4.0mm);

// Access the part_id attribute like a property
std::debug::assert(cube.part_id == 42);
```

In the example above, we attach the part id `42` to the cube and access the part id value.

Syntactically, an attribute consists of `#` prefix and an attribute item.
An attribute item can be a *tag*, a *name-value* pair or a *call*.
This results in three types of attributes: *tag attributes*, *name-value attributes* and *call attributes*.

## Tag attributes

Tag attributes assign a tag to a statement.
The following example with assign a tag `aux` to a circle:

```µcad,attributes_tag
#[aux]
std::geo2d::circle(r = 42.0mm);
```

Object nodes marked with `aux` tag will be visible in the viewer but will not be exported.

Currently, only `aux` is available as built-in tag attribute.

## Name-value attributes

Name-value attributes attach an object node with metadata as a key value list. 
There can be multiple attributes:
attribute = { "#[" ~ attribute_item ~ ws* ~ ("," ~ ws* ~ attribute_item ~ ws*)* ~ "]" }

```µcad,attributes_name_value
// Assign the sphere to `layer_a` and assign blue as color 
#[layer = "layer_a"]
#[color = "blue"]
std::geo3d::sphere(r = 4.0mm);
```

The following built-in name-value attributes are available:

* `layer`: Assign the object and its children to a layer. This is useful to distinguish object nodes by layer.
* `part_id`: Assign a part id to the object node. This is useful to map build a bill-of-materials.
* `color`: Assign a color to the object, displayed in the viewer.

## Call attributes

Call attributes are instructions to the interpreter.
For example, the `export` call attribute instructs the interpreter to export this node to the file `cube.stl`.

```µcad,attributes_call
#[export("cube.stl")]
std::geo3d::cube(4.0mm);
```

Currently, only `export` is available as built-in call attribute.

## Attribute contexts

Attributes are only valid in certain contexts.
Declaring attributes in unsupported contexts is not valid.

The following statements can have attributes:

* *expression statement*: Add an attribute the the expression. Only valid if the expression evaluates to an *object node*.

  This circle will be red:

  ```µcad,attributes_expression
  #[color = "red"]
  std::geo2d::circle(2mm);
  ```

* *assignment statement*: Add an attribute to the assignment. Only valid of the result of the assignment is an *object node*.

  The variable `rect` will contain a blue rectangle:

  ```µcad,attributes_assignment
  #[color = "blue"] 
  rect = std::geo2d::rect(2mm);
  ```

* *module definition*: Add an attribute to all nodes created by this module definition. 

  For example, all instances created by this module have the part ID `MYPART`:
  
  ```µcad,attributes_module_definition
  #[part_id = "MYPART"] 
  module my_part() { 
    std::geo2d::rect(2mm); 
  }
  ```

### Variable capture

Attributes can capture variables that are part of the current scope:

```µcad,attributes_capture
id = 32;
#[layer_id = id]
rect = std::geo2d::rect(3.0mm);
```

The means you can parametrise attributes in modules:

```µcad,attributes_module_export
module export(filename: String) {
    #[export(filename)] {
        @children
    }
}
```
