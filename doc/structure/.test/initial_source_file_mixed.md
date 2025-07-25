# Test [`initial_source_file_mixed`](/doc/structure/source_files.md#L41)

## Code

```µcad
std::geo2d::circle(radius = 1cm);
std::geo3d::sphere(radius = 1cm);  // error: can't mix 2D and 3D

```

## Output

```,plain
```

## Errors

```,plain
error: Cannot mix 2d and 3d geometries
  ---> <from_str>:2:1
     |
   2 | std::geo3d::sphere(radius = 1cm);  // error: can't mix 2D and 3D
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
```

## Test Result

![FAILED AS EXPECTED](/doc/structure/.test/initial_source_file_mixed.png)
