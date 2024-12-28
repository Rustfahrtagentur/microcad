# Diagnostics

## Contents

- [Diagnostics](#diagnostics)
  - [Contents](#contents)
  - [Helpful error messages](#helpful-error-messages)
    - [Module name without parameter list](#module-name-without-parameter-list)
    - [Calling a module like a function](#calling-a-module-like-a-function)

## Helpful error messages

This section lists some examples which all are failing with a specific error message.

### Module name without parameter list

[![test](.test/diag_module_name_without_parameter_list.png)](.test/diag_module_name_without_parameter_list.log)

```µcad,diag_module_name_without_parameter_list#fail
module f {}
```

### Calling a module like a function

[![test](.test/diag_calling_a_module_like_a_function.png)](.test/diag_calling_a_module_like_a_function.log)

```µcad,diag_calling_a_module_like_a_function#fail
module f() {}

x = f() * 2;
```
