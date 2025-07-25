# Test [`argument_match_name`](/doc/structure/arguments.md#L23)

## Code

```µcad
fn f( x: Length, y: Length, z: Length ) {}

f(x = 1cm, y = 2cm, z = 3cm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/argument_match_name.png)
