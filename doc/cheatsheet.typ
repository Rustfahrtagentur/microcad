#set page("a4", flipped: true, margin: (x: 0.5cm, y: 0.5cm), columns: 3)

#set text(font: "JetBrains Mono", size: 10pt)

= µCAD Cheatsheet

#set block(
  fill: luma(230),
  inset: 8pt,
  radius: 8pt,
  width: 100%,
)

#let section(header) = [ ]

#block[
  == Basic syntax

  use std::geo2d::\*; \/\/ Use statement

  Circle(r = 40mm); \/\/ Expression

  \#[export = "example.stl"] \/\/ Attribute

  rect = Rect(40mm); \/\/ Assignment

  use Rect as R; \/\/ Name alias

  \/\/ Apply *rotate* operation by 45°
  R(40mm).rotate(45°);

  \/\/ Multiplicity for translate

  R(40mm).translate(x = [-40, 40]mm);

  \/\/ If condition

  if a > 5mm { R(5mm) } else { R(10mm) }

]

#block[
  == Workbenches

  *mod* my_module {

  #box() *part* Part(a: Length) { ... }

  #box() *sketch* Sketch(a: Length) {

  #box(width: 12pt) \/\/ Initializer

  #box(width: 12pt) init(b: Length) { a = 2*b; }

  #box(width: 12pt) \/\/ Output geometry

  #box(width: 12pt) std::geo2d::Circle(r = a);

  #box() }

  #box() *op* operation(a: Length) {

  #box(width: 12pt) \@children.translate(x = a)

  #box() }

  }
]

#block[
  == std::geo2d

  Circle(r = 40mm);

  Rect(width = 3.0mm, height = 4.0mm);
]


#block[
  == std::geo3d

  Cylinder(r = 42mm, h = 20mm);

  Sphere(r = 50mm);

  Cube(50mm);
]

#block[
  == std::ops

  .union() \/\/ | operator

  .intersect() \/\/ & operator

  .subtract() \/\/ - operator

  .hull() \/\/ Convex hull

  .translate(x = 0mm,y = 1mm,z = 2mm)

  .rotate(45°)

  .rotate(45°, std::math::X)

  .rotate(x = 30°, y = 10°)

  .scale(2.0)

  .scale(x = 1.0,y = 2.0,z = 3.0)

  .orient(std::math::Y)

  .mirror(-std::math::Z)

  .center()

  .extrude(height = 3.0mm)

  .revolve()

  .revolve(180°)
]

#block[
  == std::math
  const PI = 3.14159;

  const X = (1,0,0);

  const Y = (0,1,0);

  const Z = (0,0,1);

  abs(x);

  sin(x);

  cos(x);

  tan(x);
]

#block[
  == std::debug

  a = 40mm;

  print("Hello World: {a}");

  assert(a > 30mm);

  assert_eq([a, 40mm]);


]


