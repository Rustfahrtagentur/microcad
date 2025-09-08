# Tests

- [Statement Usage](#statement-usage)
  - [Source](#source)
  - [Module](#module)
  - [Pre-Init](#pre-init)
  - [Init](#init)
  - [Workbench](#workbench)
  - [Body](#body)
  - [Function](#function)

## Statement Usage

### Source

[![test](.test/source_workbench.svg)](.test/source_workbench.log)

```µcad,source_workbench
sketch F() {} F();
```

[![test](.test/source_module.svg)](.test/source_module.log)

```µcad,source_module
mod m {}
```

[![test](.test/source_function.svg)](.test/source_function.log)

```µcad,source_function
fn f() {} f();
```

[![test](.test/source_init.svg)](.test/source_init.log)

```µcad,source_init#fail
init() {} // error
```

[![test](.test/source_use.svg)](.test/source_use.log)

```µcad,source_use
use std;
```

[![test](.test/source_pub_use.svg)](.test/source_pub_use.log)

```µcad,source_pub_use
pub use std;
```

[![test](.test/source_return.svg)](.test/source_return.log)

```µcad,source_return#fail
return 1;  // error
```

[![test](.test/source_if.svg)](.test/source_if.log)

```µcad,source_if
if std::math::PI == 3 { __builtin::geo2d::Circle(radius=1); }
```

[![test](.test/source_assignment_const.svg)](.test/source_assignment_const.log)

```µcad,source_assignment_const
const B = 1;
```

[![test](.test/source_assignment_var.svg)](.test/source_assignment_var.log)

```µcad,source_assignment_var
a = 1;
```

[![test](.test/source_assignment_prop.svg)](.test/source_assignment_prop.log)

```µcad,source_assignment_prop#fail
prop a = 1;  // error
```

[![test](.test/source_expression.svg)](.test/source_expression.log)

```µcad,source_expression
1 + 2;
```

[![test](.test/source_expression_model.svg)](.test/source_expression_model.log)

```µcad,source_expression_model
__builtin::geo2d::Circle(radius=1);
```

### Module

[![test](.test/module_workbench.svg)](.test/module_workbench.log)

```µcad,module_workbench
mod k {
  sketch F() {}
}
```

[![test](.test/module_module.svg)](.test/module_module.log)

```µcad,module_module
mod k {
  mod m {}
}
```

[![test](.test/module_function.svg)](.test/module_function.log)

```µcad,module_function
mod k {
  fn f() {}
}
```

[![test](.test/module_init.svg)](.test/module_init.log)

```µcad,module_init#fail
mod k {
  init() { }  // error
}
```

[![test](.test/module_use.svg)](.test/module_use.log)

```µcad,module_use
mod k {
  use std::geo2d;
}
```

[![test](.test/module_pub_use.svg)](.test/module_pub_use.log)

```µcad,module_pub_use
mod k {
  pub use std::geo2d;
}
```

[![test](.test/module_return.svg)](.test/module_return.log)

```µcad,module_return#fail
mod k {
  return 1;  // error
}
```

[![test](.test/module_if.svg)](.test/module_if.log)

```µcad,module_if#fail
mod k {
  if std::math::PI == 3 { __builtin::geo2d::Circle(radius=1); }  // error
}
```

[![test](.test/module_assignment_const.svg)](.test/module_assignment_const.log)

```µcad,module_assignment_const
mod k {
  const B = 1;
}
```

[![test](.test/module_assignment_var.svg)](.test/module_assignment_var.log)

```µcad,module_assignment_var#fail
mod k {
  a = 1; // error
}
```

[![test](.test/module_assignment_prop.svg)](.test/module_assignment_prop.log)

```µcad,module_assignment_prop#fail
mod k {
  prop a = 1;  // error
}
```

[![test](.test/module_expression.svg)](.test/module_expression.log)

```µcad,module_expression#fail
mod k {
  1 + 2; // error
}
```

[![test](.test/module_expression_model.svg)](.test/module_expression_model.log)

```µcad,module_expression_model#fail
mod k {
  __builtin::geo2d::Circle(radius=1); // error
}
```

### Pre-Init

[![test](.test/pre_init_workbench.svg)](.test/pre_init_workbench.log)

```µcad,pre_init_workbench#fail
sketch K() { 
  sketch F() {} K();  // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_module.svg)](.test/pre_init_module.log)

```µcad,pre_init_module#fail
sketch K() { 
  mod m {}   // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_function.svg)](.test/pre_init_function.log)

```µcad,pre_init_function#fail
sketch K() { 
  fn f() {} f();   // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_init.svg)](.test/pre_init_init.log)

```µcad,pre_init_init
sketch K() { 
  init() {}
init(l:Length) {} } K();
```

[![test](.test/pre_init_use.svg)](.test/pre_init_use.log)

```µcad,pre_init_use
sketch K() { 
  use std;
init(l:Length) {} } K();
```

[![test](.test/pre_init_pub_use.svg)](.test/pre_init_pub_use.log)

```µcad,pre_init_pub_use#fail
sketch K() { 
  pub use std; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_return.svg)](.test/pre_init_return.log)

```µcad,pre_init_return#fail
sketch K() { 
  return 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_if.svg)](.test/pre_init_if.log)

```µcad,pre_init_if#fail
sketch K() { 
  if std::math::PI == 3 { } // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_const.svg)](.test/pre_init_assignment_const.log)

```µcad,pre_init_assignment_const
sketch K() { 
  const B = 1;
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_var.svg)](.test/pre_init_assignment_var.log)

```µcad,pre_init_assignment_var#fail
sketch K() { 
  a = 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_prop.svg)](.test/pre_init_assignment_prop.log)

```µcad,pre_init_assignment_prop#fail
sketch K() {
  prop a = 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_expression.svg)](.test/pre_init_expression.log)

```µcad,pre_init_expression#fail
sketch K() { 
  1 + 2; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_expression_model.svg)](.test/pre_init_expression_model.log)

```µcad,pre_init_expression_model#fail
sketch K() { 
  __builtin::geo2d::Circle(radius=1); // error
init(l:Length) {} }
```

### Init

[![test](.test/init_workbench.svg)](.test/init_workbench.log)

```µcad,init_workbench#fail
sketch K() { init(l:Length) {
  sketch F() {} // error
} } K(1cm);
```

[![test](.test/init_module.svg)](.test/init_module.log)

```µcad,init_module#fail
sketch K() { init(l:Length) {
  mod m {} // error
} } K(1cm);
```

[![test](.test/init_function.svg)](.test/init_function.log)

```µcad,init_function#fail
sketch K() { init(l:Length) {
  fn f() {} // error
} } K(1cm);
```

[![test](.test/init_init.svg)](.test/init_init.log)

```µcad,init_init#fail
sketch K() { init(l:Length) {
  init() {} // error
} } K(1cm);
```

[![test](.test/init_use.svg)](.test/init_use.log)

```µcad,init_use
sketch K() { init(l:Length) {
  use std;
} } K(1cm);
```

[![test](.test/init_pub_use.svg)](.test/init_pub_use.log)

```µcad,init_pub_use#fail
sketch K() { init(l:Length) {
  pub use std; // error
} } K(1cm);
```

[![test](.test/init_return.svg)](.test/init_return.log)

```µcad,init_return#fail
sketch K() { init(l:Length) {
  return 1; // error
} } K(1cm);
```

[![test](.test/init_if.svg)](.test/init_if.log)

```µcad,init_if#fail
sketch K() { init(l:Length) {
  if std::math::PI == 3 { } // error
} } K(1cm);
```

[![test](.test/init_assignment_const.svg)](.test/init_assignment_const.log)

```µcad,init_assignment_const#fail
sketch K() { init(l:Length) {
  const B = 1; // error
} } K(1cm);
```

[![test](.test/init_assignment_var.svg)](.test/init_assignment_var.log)

```µcad,init_assignment_var
sketch K() { init(l:Length) {
  a = 1;
} } K(1cm);
```

[![test](.test/init_assignment_prop.svg)](.test/init_assignment_prop.log)

```µcad,init_assignment_prop#fail
sketch K() { init(l:Length) {
  prop a = 1; // error
} } K(1cm);
```

[![test](.test/init_expression.svg)](.test/init_expression.log)

```µcad,init_expression#fail
sketch K() { init(l:Length) {
  1 + 2; // error
} } K(1cm);
```

[![test](.test/init_expression_model.svg)](.test/init_expression_model.log)

```µcad,init_expression_model#fail
sketch K() { init(l:Length) {
  __builtin::geo2d::Circle(radius=1); // error
} } K(1cm);
```

### Workbench

[![test](.test/workbench_workbench.svg)](.test/workbench_workbench.log)

```µcad,workbench_workbench#fail
sketch K() {
  sketch F() {} F(); // error
} K();
```

[![test](.test/workbench_module.svg)](.test/workbench_module.log)

```µcad,workbench_module#fail
sketch K() {
  mod m {} // error
} K();
```

[![test](.test/workbench_function.svg)](.test/workbench_function.log)

```µcad,workbench_function
sketch K() {
  fn f() {} f();
} K();
```

[![test](.test/workbench_init.svg)](.test/workbench_init.log)

```µcad,workbench_init
sketch K() {
  init() {}
} K();
```

[![test](.test/workbench_use.svg)](.test/workbench_use.log)

```µcad,workbench_use
sketch K() {
  use std;
} K();
```

[![test](.test/workbench_pub_use.svg)](.test/workbench_pub_use.log)

```µcad,workbench_pub_use#fail
sketch K() {
  pub use std; // error
} K();
```

[![test](.test/workbench_return.svg)](.test/workbench_return.log)

```µcad,workbench_return#fail
sketch K() {
  return 1; // error
} K();
```

[![test](.test/workbench_if.svg)](.test/workbench_if.log)

```µcad,workbench_if
sketch K() {
  if std::math::PI == 3 { }
} K();
```

[![test](.test/workbench_marker.svg)](.test/workbench_marker.log)

```µcad,workbench_marker
sketch K() {
  @input
} K();
```

[![test](.test/workbench_assignment_const.svg)](.test/workbench_assignment_const.log)

```µcad,workbench_assignment_const
sketch K() {
  const B = 1;
} K();
```

[![test](.test/workbench_assignment_var.svg)](.test/workbench_assignment_var.log)

```µcad,workbench_assignment_var
sketch K() {
  a = 1;
} K();
```

[![test](.test/workbench_assignment_prop.svg)](.test/workbench_assignment_prop.log)

```µcad,workbench_assignment_prop
sketch K() {
  prop a = 1;
} K();
```

[![test](.test/workbench_expression.svg)](.test/workbench_expression.log)

```µcad,workbench_expression
sketch K() {
  1 + 2;
} K();
```

[![test](.test/workbench_expression_model.svg)](.test/workbench_expression_model.log)

```µcad,workbench_expression_model
sketch K() {
  __builtin::geo2d::Circle(radius=1);
} K();
```

### Body

[![test](.test/body_workbench.svg)](.test/body_workbench.log)

```µcad,body_workbench#fail
{
  sketch F() {} F(); // error
}
```

[![test](.test/body_module.svg)](.test/body_module.log)

```µcad,body_module#fail
{
  mod m {} // error
}
```

[![test](.test/body_function.svg)](.test/body_function.log)

```µcad,body_function#fail
{
  fn f() {} f(); // error
}
```

[![test](.test/body_init.svg)](.test/body_init.log)

```µcad,body_init#fail
{
  init() {} // error
}
```

[![test](.test/body_use.svg)](.test/body_use.log)

```µcad,body_use
{
  use std;
}
```

[![test](.test/body_pub_use.svg)](.test/body_pub_use.log)

```µcad,body_pub_use#fail
{
  pub use std; // error
}
```

[![test](.test/body_return.svg)](.test/body_return.log)

```µcad,body_return#fail
{
  return 1; // error
}
```

[![test](.test/body_if.svg)](.test/body_if.log)

```µcad,body_if
{
  if std::math::PI == 3 { }
}
```

[![test](.test/body_marker.svg)](.test/body_marker.log)

```µcad,body_marker
{
  @input
}
```

[![test](.test/body_assignment_const.svg)](.test/body_assignment_const.log)

```µcad,body_assignment_const#fail
{
  const B = 1; // error
}
```

[![test](.test/body_assignment_var.svg)](.test/body_assignment_var.log)

```µcad,body_assignment_var
{
  a = 1;
}
```

[![test](.test/body_assignment_prop.svg)](.test/body_assignment_prop.log)

```µcad,body_assignment_prop#fail
{
  prop a = 1; // error
}
```

[![test](.test/body_expression.svg)](.test/body_expression.log)

```µcad,body_expression
{
  1 + 2;
}
```

[![test](.test/body_expression_model.svg)](.test/body_expression_model.log)

```µcad,body_expression_model
{
  __builtin::geo2d::Circle(radius=1);
}
```

### Function

[![test](.test/function_workbench.svg)](.test/function_workbench.log)

```µcad,function_workbench#fail
fn f() {
  sketch S() {} // error
} f();
```

[![test](.test/function_module.svg)](.test/function_module.log)

```µcad,function_module#fail
fn f() {
  mod m {} // error
} f();
```

[![test](.test/function_function.svg)](.test/function_function.log)

```µcad,function_function#fail
fn f() {
  fn f() {} // error
} f();
```

[![test](.test/function_init.svg)](.test/function_init.log)

```µcad,function_init#fail
fn f() {
  init() {} // error
} f();
```

[![test](.test/function_use.svg)](.test/function_use.log)

```µcad,function_use
fn f() {
  use std;
} f();
```

[![test](.test/function_pub_use.svg)](.test/function_pub_use.log)

```µcad,function_pub_use#fail
fn f() {
  pub use std; // error
} f();
```

[![test](.test/function_return.svg)](.test/function_return.log)

```µcad,function_return
fn f() {
  return 1;
} f();
```

[![test](.test/function_if.svg)](.test/function_if.log)

```µcad,function_if
fn f() {
  if std::math::PI == 3 { __builtin::geo2d::Circle(radius=1); }
} f();
```

[![test](.test/function_assignment_const.svg)](.test/function_assignment_const.log)

```µcad,function_assignment_const
fn f() {
  const B = 1;
} f();
```

[![test](.test/function_assignment_var.svg)](.test/function_assignment_var.log)

```µcad,function_assignment_var
fn f() {
  a = 1;
} f();
```

[![test](.test/function_assignment_prop.svg)](.test/function_assignment_prop.log)

```µcad,function_assignment_prop#fail
fn f() {
  prop a = 1; // error
} f();
```

[![test](.test/function_expression.svg)](.test/function_expression.log)

```µcad,function_expression
fn f() {
  1 + 2;
} f();
```

[![test](.test/function_expression_model.svg)](.test/function_expression_model.log)

```µcad,function_expression_model
fn f() {
  __builtin::geo2d::Circle(radius=1);
} f();
```
