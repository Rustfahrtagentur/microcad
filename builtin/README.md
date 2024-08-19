# Built-in proc macro

This proc macro generates built-in modules from structs.

## Example

Let's look at µCAD's built-in module `rect`.
It has the following parameter signature: `rect(width: scalar, height: scalar, x: scalar y: scalar)`.

Notice that the parameters are all of type `scalar` and not `length`.
This is because the built-in module is not aware of the unit system.
The built-in modules are not meant to be used directly.
Instead, they are wrapped in a module written µCAD language that provides units, asserts, default values and multiple initializers.

In Rust, a built-in module is defined by a struct that implements the `BuiltInModule` trait.
For our `rect` module, the struct looks like this:

```rust
#[derive(DefineBuiltInModule)]
struct Rect {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}
```

The `DefineBuiltInModule` trait is defined as follows:

```rust
pub trait DefineBuiltInModule {
    fn name() -> &'static str;
    fn parameters() -> ParameterList;
    fn function() -> &'static BuiltInModuleFn;

    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            name: Self::name(),
            parameters: Self::parameters(),
            function: Self::function(),
        }
    }
}
```

For the `rect` module, the implementation of the `DefineBuiltInModule` trait looks like this approximately:

```rust
impl DefineBuiltInModule for Rectangle {
    fn name() -> &'static str {
        "rect"
    }

    fn parameters() -> ParameterList {
        parameter_list![
            parameter!(width: Scalar),
            parameter!(height: Scalar),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
        ]
    }

    fn function() -> &'static BuiltInModuleFn {
        |args, ctx| {
            let node = Node::new(NodeInner::Generator2D(Box::new(Rect {
                width: args["width"].clone().try_into()?,
                height: args["height"].clone().try_into()?,
                x: args["x"].clone().try_into()?,
                y: args["y"].clone().try_into()?,
            })));
            Ok(ctx.append_node(node))
        }
    }
}
```
