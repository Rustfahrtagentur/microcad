# Unit Types

Units and types are somehow the same in *µcad*.
This means that your will automatically get the following type if you use the beside units.

| Type      | Metric Units                         | Imperial Units |
| --------- | ------------------------------------ | -------------- |
| `Length`  | `m`, `cm`, `mm`, `µm`                | `in`           |
| `Angle`   | `°`, `deg`, `grad`, `turn`,`rad`     |                |
| `Weight`  | `g`, `kg`                            | `lb`           |
| `Area`    | `mm²`,`cm²`,`m³`                     | `in²`          |
| `Volume`  | `mm³`,`cm³`,`m³`,`ml`,`cl`,`l`, `µl` | `in³`          |
| `Integer` | -                                    | -              |
| `Scalar`  | (none), `%`                          | -              |

**Note**: More units [will be implemented](https://github.com/Rustfahrtagentur/microcad/issues/76).

## Usage

*Types* are just used in *parameter declarations* while *Units* are widely used in *literal values* within *expressions* or to set *defaults* of parameters in fun.

### Types

![test](.test/README_types.png)

```µcad,README_types
// function parameter `height` declared to be a `Length`
function f( height: Length ) {}
```

### Units

![test](.test/README_number_literals.png)

```µcad,README_number_literals
// declare variable `height` of type `Length` to 1.4 Meters
height = 1.4m;

// use as *default* value in parameter list
function f( height = 1.0m ) {}

// calculate a `Length` called `width` by multiplying the
// `height` with `Scalar` `2.0` and add ten centimeters
width = height * 2.0 + 10cm;
```
