#set page("a4", flipped: true, margin: (x: 0.5cm, y: 0.5cm), columns: 3)

#set text(font: "JetBrains Mono", size: 10pt)


#image("images/logo.png", width: 50%)
= Cheatsheet (v0.2.0)

#set block(
  fill: luma(230),
  inset: 6pt,
  radius: 8pt,
  width: 100%,
)
#let section(header) = [ ]


#block[
  == Command line usage

  #set par(spacing: 3.3mm)

  \# Print syntax tree

  *microcad parse file.µcad*

  \# Print model tree

  *microcad eval file.µcad*

  \# Export geometry as STL or SVG

  *microcad export file.µcad*

  \# Update file on changes

  *microcad watch file.µcad*
]


#block[
  == Basic syntax

  #set par(spacing: 3.4mm)

  *use* std::geo2d::\*; \/\/ Use statement

  *use* Rect *as* R; \/\/ Name alias

  *pub use* Rect *as* R; \/\/ Public name alias

  Circle(r = 40mm); \/\/ Sketch/part call

  \#[export = "example.stl"] \/\/ Attribute

  rect = R(40mm); \/\/ Assignment

  *const* rect = R(4mm); \/\/ Private value

  *pub* rect = R(40mm); \/\/ Public value

  *prop* rect = R(40mm); \/\/ Property

  \/\/ Apply *rotate* operation by 45°
  R(40mm).rotate(45°);

  \/\/ Multiplicity for translate

  R(40mm).translate(x = [-40, 40]mm);

  { R(40mm); Circle(r = 4mm); } \/\/ Group

  \/\/ If condition

  if a > 5mm { R(5mm) } else { R(10mm) }
]


#block[
  == Workbenches

  #set par(spacing: 2.8mm)

  *mod* file; \/\/ Include µcad file.

  *mod* my_module {

  #box() *part* Part(a: Length) { ... }

  #box() *sketch* Sketch(a: Length) {

  #box(width: 12pt) \/\/ Initializer

  #box(width: 12pt) init(b: Length) { a = 2*b; }

  #box(width: 12pt) \/\/ Output geometry

  #box(width: 12pt) std::geo2d::Circle(r = a);

  #box() }

  #box() *op* operation(a: Length) {

  #box(width: 12pt) \@input.translate(x = a)

  #box() }

  }
]

#block[
  == Functions

  *fn* function(x: Length) {

  #box() *return* x; \/\/ Return statement

  }

]


#block[
  == std::geo2d

  #set par(spacing: 2.2mm)

  Circle(r = 40mm);

  Rect(width = 3.0mm, height = 4.0mm);

  Line(p0 = (x = 0.0, y = 0.0), \ #box(width: 8.4mm) p1 = (x = 10.0, y = 20.0));

  Frame(width = 2mm,\ #box(width: 10.2mm) height = 4mm, thickness = 1mm);

  Ring(outer_r = 50mm, inner_r = 40mm);
]


#block[
  == std::geo3d
  #set par(spacing: 2.2mm)

  Cylinder(r = 42mm, h = 20mm);

  Sphere(r = 50mm);

  Cube(50mm);
]


#block[
  == std::debug
  #set par(spacing: 2.2mm)

  a = 40mm;

  print("Hello World: {a}");

  assert(a > 30mm);

  assert_eq([a, 40mm]);

]

#block[
  == std::math
  const PI = 3.141592653589793238462643;

  const X = (1,0,0);

  const Y = (0,1,0);

  const Z = (0,0,1);

  abs(x); sin(x); cos(x); tan(x);
]


#block[
  == std::ops

  #set par(spacing: 2.85mm)

  .union() \/\/ | operator

  .intersect() \/\/ & operator

  .subtract() \/\/ - operator

  .hull() \/\/ Convex hull

  .translate(x = 0mm,y = 1mm,z = 2mm)

  .rotate(45°) \/\/ 2D rotate / rotate around Z

  .rotate(45°, std::math::X)

  .rotate(x = 30°, y = 10°)

  .scale(2.0) \/\/ Uniform scale

  .scale(x = 1.0, y = 2.0, z = 3.0)

  .orient(std::math::Y) \/\/ Orient along axis

  .mirror(-std::math::Z)

  .align() \/\/ Align at center

  .extrude(height = 3.0mm) \/\/ Linear extrude

  .revolve() \/\/ Revolve extrude

  .revolve(180°)
]


