# Import data

Use can import data via `std::import` function.

*Note: This WIP. Currently, only tuples from a TOML file can be imported.*


## TOML import

Assuming, you have the following data in a TOML file `example.toml`:

```toml
[M6]
diameter = 6.0
pitch = 1.0

[M10]
diameter = 10.0
pitch = 1.5
```

You can then load the TOML file by using `std::import` and access its values:

```µcad,toml_import
data = std::import("example.toml");

std::debug::assert_eq(data.M10.diameter, 10.0);
std::debug::assert_eq(data.M6.pitch, 1.0);
```

