# Test [`code_between_initializers`](/doc/structure/workbench.md#L203)

## Code

```µcad
sketch wheel(radius: Length) {
    init( width:Length ) { radius = width / 2; }
    
    // error: code between initializers not allowed
    radius = 1;

    init( height:Length ) { radius = height / 2; }
}

wheel(radius = 1.0mm);

```

## Parse Error

```,plain
Code between initializers is not allowed```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/code_between_initializers.png)
