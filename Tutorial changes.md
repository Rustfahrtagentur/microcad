# Änderungen Tutorial

- Tutorial in mehrere Dateien Einteilen, die mit Pfeile verkettet werden

1. Introduction
   1. What will we do in this tutorial?
   2. Needed skills
   3. Needed Time
   4. Learning objectives
2. How to construct a Lego brick?
   1. Maße besorgen
      1. link
   2. first construct a 4x2 brick
   3. Divide into components: Base, Cap, Knobs
   4. First make sketches and then assemble to a part
   5. make a customizable part
3. Preparation
   1. install microcad
   2. create microcad file
4. Sketch "Frame"
   1. Sketch of the "Inner Frames"
      1. std::geo3d::Rect
      2. rename: `wall_width` -> `thickness`
   2. Sketch of the "Outer Frames"
   3. Output both frames
      1. rename: `wall_width` -> `thickness`
   4. Operations
      1. std::ops::translate
      2. std::ops::rotate
   5. use statements
      1. shorten code
   6. Grouping Statements
      1. avoid assignment)here
      2. rename: `wall_width` -> `thickness`
   7. Applying operation to the group
       1. avoid assignment here
       2. rename: `wall_width` -> `thickness`
   8. Naming things with assignments
      1. tell what for (-operator))
   9. use -operator
       1. rename: `wall_width` -> `thickness`
   10. Replace code by using builtin std::geo2d::Frame
5. Sketch "Struts"
   1. Sketch of a single "Strut"
      1. aka "Creating our first sketch" and Constructing single.."
   2. Constructing multiple struts
   3. Use "Parameter Multiplicity"
   4. Constants
       1. describe: "multiply an array"
   5. Range expressions
       1. Why not use `[-1..1]` instead of `center()`?
   6. Replace code by using builtin std::geo2d::Ring
6. Combining both Elements with boolean operation `|`
   1. rename: `wall_width` -> `thickness`
7. First sketch: the "Base" (no builtin for that)
   1. Make it reusable with Workbench
   2. Create a sketch of the "Base"
   3. rename: `wall_width` -> `thickness`
8. Sketch "Cap"
   1. Just use `use as`
   2. Why do not use `std::geo3d::Cube()`?
9. Sketch "Knobs"
   1. use `Circle` and multiplicity
   2. Explain Tuples
10. Overview of the sketches
    1. Picture!
    2. use `|` to combine sketches?
    3. rename: `wall_width` -> `thickness`
11. Extrude all sketches

- if there is std::geo2d::Frame why not make std::geo2d::Ring too?
- Analogy to natural language (does not need to be explained because it is intentional)
- Default parts in std::geo3d
  