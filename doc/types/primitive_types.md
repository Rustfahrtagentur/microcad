# Primitive Types

## Bool

Boolean is the result type of boolean expressions which may just be `true` or `false`.

[![test](.test/types_primitive_bool.png)](.test/types_primitive_bool.log)

```µcad,types_primitive_bool
std::debug::assert( true != false );
```

## Integer

The type `integer` contains a natural number.

[![test](.test/types_primitive_integer.png)](.test/types_primitive_integer.log)

```µcad,types_primitive_integer
i = 3;
```

## String

Text can be used to logging or to render text.

[![test](.test/types_primitive_string.png)](.test/types_primitive_string.log)

```µcad,types_primitive_string
text = "Hello µcad!";
std::debug::assert(text.count() == 11);

// logging
std::print(text);
```

## Matrix

Matrix types are built-in types and can be defined as:

- `Matrix2`: A 2x2 matrix, commonly used for 2D rotations.
- `Matrix3`: A 3x3 matrix, commonly used for rotations.
- `Matrix4`: A 4x4 matrix, commonly used for affine transformations.

*Note: Currently, matrices are used only internally and cannot be instantiated from µcad code.*
