# Unit Types

Units and types are somehow the same in *µcad*.
This means that your will automatically get the following type if you use the beside units.

| Type      | Value           | Metric Units                                | Imperial Units                 |
| --------- | --------------- | ------------------------------------------- | ------------------------------ |
| `Length`  | floating point  | `µm`, `mm`, `cm`, `m`                       | `in` or `"`, `ft` or `'`, `yd` |
| `Angle`   | floating point  | `°` or `deg`, `grad`, `turn`,`rad`          |                                |
| `Weight`  | floating point  | `g`, `kg`                                   | `lb`, `oz`                     |
| `Area`    | floating point  | `µm²`,`mm²`,`cm²`,`m³`                      | `in²`, `ft²` , `yd²`           |
| `Volume`  | floating point  | `µm³`, `mm³`,`cm³`,`m³`,`ml`,`cl`,`l`, `µl` | `in³`, `ft³` , `yd³`           |
| `Integer` | signed  integer | -                                           | -                              |
| `Scalar`  | floating point  | -, `%`                                      | -                              |

**Note**: More units [may be implemented](https://github.com/Rustfahrtagentur/microcad/issues/76).

## Usage

*Types* are just used in *parameter declarations* while *Units* are widely used in *literal values* within *expressions* or to set *defaults* of parameters in fun.

### Types

![test](.test/README_types.png)
[see build log](.test/README_types.log)

```µcad,README_types
// function parameter `height` declared to be a `Length`
function f( height: Length ) {}
```

### Units

![test](.test/README_number_literals.png)
[see build log](.test/README_number_literals.log)

```µcad,README_number_literals
// declare variable `height` of type `Length` to 1.4 Meters
height = 1.4m;

// use as *default* value in parameter list
function f( height = 1m ) {}

// calculate a `Length` called `width` by multiplying the
// `height` with `Scalar` `2` and add ten centimeters
width = height * 2 + 10cm;
```
