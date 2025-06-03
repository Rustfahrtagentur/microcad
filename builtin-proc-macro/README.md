# Built-in proc macro

This proc macro generates built-in modules from structs.

## Example

Let's look at µcad's built-in part `rect`.
It has the following parameter signature: `rect(width: Scalar, height: Scalar, x: Scalar y: Scalar)`.

Notice that the parameters are all of type `Scalar` and not `Length`.
This is because the built-in library is not aware of the unit system.
The built-in modules are not meant to be used directly.
Instead, they are wrapped in a part or module written in µcad language that provides units, asserts, default values and multiple initializers.

In Rust, a built-in library is defined by a struct that implements the `Builtin` trait.
For our `rect` part, the struct looks like this:

```rust
#[derive(DefineBuiltinModule)]
struct Rect {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}
```

The `DefineBuiltinModule` trait is defined as follows:

```rust
pub trait DefineBuiltinModule {
    fn name() -> &'static str;
    fn parameters() -> ParameterList;
    fn node(args: &ArgumentMap) -> Node;
    fn function() -> &'static BuiltinModuleFn { ... }
    }

    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            name: Self::name(),
            parameters: Self::parameters(),
            function: Self::function(),
        }
    }
}
```

For the `rect` part, the implementation of the `DefineBuiltinModule` trait looks like this approximately:

```rust
impl DefineBuiltinModule for Rectangle {
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

    fn node(args: &ArgumentMap) -> Result<Node, Error> {
        Ok(Node::new(NodeInner::Generator2D(Box::new(Rect {
            width: args["width"].try_into()?,
            height: args["height"].try_into()?,
            x: args["x"].try_into()?,
            y: args["y"].try_into()?,
        })))
    }
}
```
