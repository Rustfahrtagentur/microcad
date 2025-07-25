# Test [`example`](/doc/structure/functions.md#L14)

## Code

```µcad
// define function print_error with text as parameter of type String
fn print_error( text: String ) {
    // code body
    std::print("ERROR: {text}");
}

print_error("first");
print_error("second");

```

## Output

```,plain
ERROR: first
ERROR: second
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/example.png)
