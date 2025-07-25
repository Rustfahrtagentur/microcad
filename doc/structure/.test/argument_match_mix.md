# Test [`argument_match_mix`](/doc/structure/arguments.md#L48)

## Code

```µcad
fn f( a: Scalar, b: Length, c: Length ) {}
// `a` is the only Scalar and `b` is named, so `c` does not need a name.
f(1.0, b=2cm, 3cm);

```

## Output

```,plain
```

## Errors

```,plain
```

## Test Result

![OK](/doc/structure/.test/argument_match_mix.png)
