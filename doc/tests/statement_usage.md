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

[![test](.test/source_workbench.png)](.test/source_workbench.md)

```µcad,source_workbench
sketch f() {} f();
```

[![test](.test/source_module.png)](.test/source_module.md)

```µcad,source_module
mod m {}
```

[![test](.test/source_function.png)](.test/source_function.md)

```µcad,source_function
fn f() {} f();
```

[![test](.test/source_init.png)](.test/source_init.md)

```µcad,source_init#fail
init() {}
```

[![test](.test/source_use.png)](.test/source_use.md)

```µcad,source_use
use std;
```

[![test](.test/source_pub_use.png)](.test/source_pub_use.md)

```µcad,source_pub_use
pub use std;
```

[![test](.test/source_return.png)](.test/source_return.md)

```µcad,source_return#fail
return 1;
```

[![test](.test/source_if.png)](.test/source_if.md)

```µcad,source_if
if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
```

[![test](.test/source_marker.png)](.test/source_marker.md)

```µcad,source_marker#fail
@children
```

[![test](.test/source_assignment_const.png)](.test/source_assignment_const.md)

```µcad,source_assignment_const
const B = 1;
```

[![test](.test/source_assignment_var.png)](.test/source_assignment_var.md)

```µcad,source_assignment_var
a = 1;
```

[![test](.test/source_assignment_prop.png)](.test/source_assignment_prop.md)

```µcad,source_assignment_prop#fail
prop a = 1;
```

[![test](.test/source_expression.png)](.test/source_expression.md)

```µcad,source_expression#fail
1 + 2;
```

[![test](.test/source_expression_model.png)](.test/source_expression_model.md)

```µcad,source_expression_model
__builtin::geo2d::circle(radius=1);
```

### Module

[![test](.test/module_workbench.png)](.test/module_workbench.md)

```µcad,module_workbench
mod k {
  sketch f() {}
}
```

[![test](.test/module_module.png)](.test/module_module.md)

```µcad,module_module
mod k {
  mod m {}
}
```

[![test](.test/module_function.png)](.test/module_function.md)

```µcad,module_function
mod k {
  fn f() {}
}
```

[![test](.test/module_init.png)](.test/module_init.md)

```µcad,module_init#fail
mod k {
  init() { }
}
```

[![test](.test/module_use.png)](.test/module_use.md)

```µcad,module_use
mod k {
  use std;
}
```

[![test](.test/module_pub_use.png)](.test/module_pub_use.md)

```µcad,module_pub_use#todo
mod k {
  pub use std;
}
```

[![test](.test/module_return.png)](.test/module_return.md)

```µcad,module_return#fail
mod k {
  return 1;
}
```

[![test](.test/module_if.png)](.test/module_if.md)

```µcad,module_if#fail
mod k {
  if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
}
```

[![test](.test/module_marker.png)](.test/module_marker.md)

```µcad,module_marker#fail
mod k {
  @children
}
```

[![test](.test/module_assignment_const.png)](.test/module_assignment_const.md)

```µcad,module_assignment_const
mod k {
  const B = 1;
}
```

[![test](.test/module_assignment_var.png)](.test/module_assignment_var.md)

```µcad,module_assignment_var
mod k {
  a = 1;
}
```

[![test](.test/module_assignment_prop.png)](.test/module_assignment_prop.md)

```µcad,module_assignment_prop#fail
mod k {
  prop a = 1;
}
```

[![test](.test/module_expression.png)](.test/module_expression.md)

```µcad,module_expression#fail
mod k {
  1 + 2;
}
```

[![test](.test/module_expression_model.png)](.test/module_expression_model.md)

```µcad,module_expression_model#fail
mod k {
  __builtin::geo2d::circle(radius=1);
}
```

### Pre-Init

[![test](.test/pre_init_workbench.png)](.test/pre_init_workbench.md)

```µcad,pre_init_workbench#fail
sketch k() { 
  sketch f() {} f();
init(l:Length) {} } k();
```

[![test](.test/pre_init_module.png)](.test/pre_init_module.md)

```µcad,pre_init_module#fail
sketch k() { 
  mod m {}
init(l:Length) {} } k();
```

[![test](.test/pre_init_function.png)](.test/pre_init_function.md)

```µcad,pre_init_function#fail
sketch k() { 
  fn f() {} f();
init(l:Length) {} } k();
```

[![test](.test/pre_init_init.png)](.test/pre_init_init.md)

```µcad,pre_init_init
sketch k() { 
  init() {}
init(l:Length) {} } k();
```

[![test](.test/pre_init_use.png)](.test/pre_init_use.md)

```µcad,pre_init_use
sketch k() { 
  use std;
init(l:Length) {} } k();
```

[![test](.test/pre_init_pub_use.png)](.test/pre_init_pub_use.md)

```µcad,pre_init_pub_use
sketch k() { 
  pub use std;
init(l:Length) {} } k();
```

[![test](.test/pre_init_return.png)](.test/pre_init_return.md)

```µcad,pre_init_return#fail
sketch k() { 
  return 1;
init(l:Length) {} } k();
```

[![test](.test/pre_init_if.png)](.test/pre_init_if.md)

```µcad,pre_init_if#fail
sketch k() { 
  if std::math::PI == 3 { }
init(l:Length) {} } k();
```

[![test](.test/pre_init_marker.png)](.test/pre_init_marker.md)

```µcad,pre_init_marker#fail
sketch k() { 
  @children
init(l:Length) {} } k();
```

[![test](.test/pre_init_assignment_const.png)](.test/pre_init_assignment_const.md)

```µcad,pre_init_assignment_const
sketch k() { 
  const B = 1;
init(l:Length) {} } k();
```

[![test](.test/pre_init_assignment_var.png)](.test/pre_init_assignment_var.md)

```µcad,pre_init_assignment_var#fail
sketch k() { 
  a = 1;
init(l:Length) {} } k();
```

[![test](.test/pre_init_assignment_prop.png)](.test/pre_init_assignment_prop.md)

```µcad,pre_init_assignment_prop#fail
sketch k() { 
  prop a = 1;
init(l:Length) {} } k();
```

[![test](.test/pre_init_expression.png)](.test/pre_init_expression.md)

```µcad,pre_init_expression#fail
sketch k() { 
  1 + 2;
init(l:Length) {} } k();
```

[![test](.test/pre_init_expression_model.png)](.test/pre_init_expression_model.md)

```µcad,pre_init_expression_model#fail
sketch k() { 
  __builtin::geo2d::circle(radius=1);
init(l:Length) {} }
```

### Init

[![test](.test/init_workbench.png)](.test/init_workbench.md)

```µcad,init_workbench#fail
sketch k() { init(l:Length) {
  sketch f() {}
} } k(1cm);
```

[![test](.test/init_module.png)](.test/init_module.md)

```µcad,init_module#fail
sketch k() { init(l:Length) {
  mod m {}
} } k(1cm);
```

[![test](.test/init_function.png)](.test/init_function.md)

```µcad,init_function#fail
sketch k() { init(l:Length) {
  fn f() {}
} } k(1cm);
```

[![test](.test/init_init.png)](.test/init_init.md)

```µcad,init_init#fail
sketch k() { init(l:Length) {
  init() {}
} } k(1cm);
```

[![test](.test/init_use.png)](.test/init_use.md)

```µcad,init_use
sketch k() { init(l:Length) {
  use std;
} } k(1cm);
```

[![test](.test/init_pub_use.png)](.test/init_pub_use.md)

```µcad,init_pub_use#todo_fail
sketch k() { init(l:Length) {
  pub use std;
} } k(1cm);
```

[![test](.test/init_return.png)](.test/init_return.md)

```µcad,init_return#fail
sketch k() { init(l:Length) {
  return 1;
} } k(1cm);
```

[![test](.test/init_if.png)](.test/init_if.md)

```µcad,init_if#fail
sketch k() { init(l:Length) {
  if std::math::PI == 3 { }
} } k(1cm);
```

[![test](.test/init_marker.png)](.test/init_marker.md)

```µcad,init_marker#fail
sketch k() { init(l:Length) {
  @children
} } k(1cm);
```

[![test](.test/init_assignment_const.png)](.test/init_assignment_const.md)

```µcad,init_assignment_const#fail
sketch k() { init(l:Length) {
  const B = 1;
} } k(1cm);
```

[![test](.test/init_assignment_var.png)](.test/init_assignment_var.md)

```µcad,init_assignment_var
sketch k() { init(l:Length) {
  a = 1;
} } k(1cm);
```

[![test](.test/init_assignment_prop.png)](.test/init_assignment_prop.md)

```µcad,init_assignment_prop#fail
sketch k() { init(l:Length) {
  prop a = 1;
} } k(1cm);
```

[![test](.test/init_expression.png)](.test/init_expression.md)

```µcad,init_expression#fail
sketch k() { init(l:Length) {
  1 + 2;
} } k(1cm);
```

[![test](.test/init_expression_model.png)](.test/init_expression_model.md)

```µcad,init_expression_model#fail
sketch k() { init(l:Length) {
  __builtin::geo2d::circle(radius=1);
} } k(1cm);
```

### Workbench

[![test](.test/workbench_workbench.png)](.test/workbench_workbench.md)

```µcad,workbench_workbench#fail
sketch k() {
  sketch f() {} f();
} k();
```

[![test](.test/workbench_module.png)](.test/workbench_module.md)

```µcad,workbench_module#fail
sketch k() {
  mod m {}
} k();
```

[![test](.test/workbench_function.png)](.test/workbench_function.md)

```µcad,workbench_function#fail
sketch k() {
  fn f() {} f();
} k();
```

[![test](.test/workbench_init.png)](.test/workbench_init.md)

```µcad,workbench_init
sketch k() {
  init() {}
} k();
```

[![test](.test/workbench_use.png)](.test/workbench_use.md)

```µcad,workbench_use
sketch k() {
  use std;
} k();
```

[![test](.test/workbench_pub_use.png)](.test/workbench_pub_use.md)

```µcad,workbench_pub_use#todo_fail
sketch k() {
  pub use std;
} k();
```

[![test](.test/workbench_return.png)](.test/workbench_return.md)

```µcad,workbench_return#fail
sketch k() {
  return 1;
} k();
```

[![test](.test/workbench_if.png)](.test/workbench_if.md)

```µcad,workbench_if
sketch k() {
  if std::math::PI == 3 { }
} k();
```

[![test](.test/workbench_marker.png)](.test/workbench_marker.md)

```µcad,workbench_marker
sketch k() {
  @children
} k();
```

[![test](.test/workbench_assignment_const.png)](.test/workbench_assignment_const.md)

```µcad,workbench_assignment_const
sketch k() {
  const B = 1;
} k();
```

[![test](.test/workbench_assignment_var.png)](.test/workbench_assignment_var.md)

```µcad,workbench_assignment_var
sketch k() {
  a = 1;
} k();
```

[![test](.test/workbench_assignment_prop.png)](.test/workbench_assignment_prop.md)

```µcad,workbench_assignment_prop
sketch k() {
  prop a = 1;
} k();
```

[![test](.test/workbench_expression.png)](.test/workbench_expression.md)

```µcad,workbench_expression#fail
sketch k() {
  1 + 2;
} k();
```

[![test](.test/workbench_expression_model.png)](.test/workbench_expression_model.md)

```µcad,workbench_expression_model
sketch k() {
  __builtin::geo2d::circle(radius=1);
} k();
```

### Body

[![test](.test/body_workbench.png)](.test/body_workbench.md)

```µcad,body_workbench#fail
{
  sketch f() {} f();
}
```

[![test](.test/body_module.png)](.test/body_module.md)

```µcad,body_module#fail
{
  mod m {}
}
```

[![test](.test/body_function.png)](.test/body_function.md)

```µcad,body_function#fail
{
  fn f() {} f();
}
```

[![test](.test/body_init.png)](.test/body_init.md)

```µcad,body_init#fail
{
  init() {}
}
```

[![test](.test/body_use.png)](.test/body_use.md)

```µcad,body_use
{
  use std;
}
```

[![test](.test/body_pub_use.png)](.test/body_pub_use.md)

```µcad,body_pub_use#todo_fail
{
  pub use std;
}
```

[![test](.test/body_return.png)](.test/body_return.md)

```µcad,body_return#fail
{
  return 1;
}
```

[![test](.test/body_if.png)](.test/body_if.md)

```µcad,body_if
{
  if std::math::PI == 3 { }
}
```

[![test](.test/body_marker.png)](.test/body_marker.md)

```µcad,body_marker
{
  @children
}
```

[![test](.test/body_assignment_const.png)](.test/body_assignment_const.md)

```µcad,body_assignment_const#fail
{
  const B = 1;
}
```

[![test](.test/body_assignment_var.png)](.test/body_assignment_var.md)

```µcad,body_assignment_var
{
  a = 1;
}
```

[![test](.test/body_assignment_prop.png)](.test/body_assignment_prop.md)

```µcad,body_assignment_prop#fail
{
  prop a = 1;
}
```

[![test](.test/body_expression.png)](.test/body_expression.md)

```µcad,body_expression
{
  1 + 2;
}
```

[![test](.test/body_expression_model.png)](.test/body_expression_model.md)

```µcad,body_expression_model
{
  __builtin::geo2d::circle(radius=1);
}
```

### Function

[![test](.test/function_workbench.png)](.test/function_workbench.md)

```µcad,function_workbench#fail
fn f() {
  sketch s() {}
} f();
```

[![test](.test/function_module.png)](.test/function_module.md)

```µcad,function_module#fail
fn f() {
  mod m {}
} f();
```

[![test](.test/function_function.png)](.test/function_function.md)

```µcad,function_function#fail
fn f() {
  fn f() {}
} f();
```

[![test](.test/function_init.png)](.test/function_init.md)

```µcad,function_init#fail
fn f() {
  init() {}
} f();
```

[![test](.test/function_use.png)](.test/function_use.md)

```µcad,function_use
fn f() {
  use std;
} f();
```

[![test](.test/function_pub_use.png)](.test/function_pub_use.md)

```µcad,function_pub_use
fn f() {
  pub use std;
} f();
```

[![test](.test/function_return.png)](.test/function_return.md)

```µcad,function_return
fn f() {
  return 1;
} f();
```

[![test](.test/function_if.png)](.test/function_if.md)

```µcad,function_if
fn f() {
  if std::math::PI == 3 { __builtin::geo2d::circle(radius=1); }
} f();
```

[![test](.test/function_marker.png)](.test/function_marker.md)

```µcad,function_marker#fail
fn f() {
  @children
} f();
```

[![test](.test/function_assignment_const.png)](.test/function_assignment_const.md)

```µcad,function_assignment_const#fail
fn f() {
  const B = 1;
} f();
```

[![test](.test/function_assignment_var.png)](.test/function_assignment_var.md)

```µcad,function_assignment_var
fn f() {
  a = 1;
} f();
```

[![test](.test/function_assignment_prop.png)](.test/function_assignment_prop.md)

```µcad,function_assignment_prop#fail
fn f() {
  prop a = 1;
} f();
```

[![test](.test/function_expression.png)](.test/function_expression.md)

```µcad,function_expression
fn f() {
  1 + 2;
} f();
```

[![test](.test/function_expression_model.png)](.test/function_expression_model.md)

```µcad,function_expression_model
fn f() {
  __builtin::geo2d::circle(radius=1);
} f();
```
