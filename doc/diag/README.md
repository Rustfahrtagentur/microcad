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

<details>
<summary>Error message should be like:</summary>

```µcad_err
error: module `f` is missing a parameter list.
  ---> <no file>:1:1
     |
   1 | x = f() * 2;
     |     ^^^^^^^
     |
```

</details>

### Calling a module like a function

[![test](.test/diag_calling_a_module_like_a_function.png)](.test/diag_calling_a_module_like_a_function.log)

```µcad,diag_calling_a_module_like_a_function#fail
module f() {}

x = f() * 2;
```

<details>
<summary>Error message should be like:</summary>

```µcad_err
error: Cannot multiply module `f()` with value `2`
  ---> <no file>:1:1
     |
   1 | x = f() * 2;
     |     ^^^^^^^
     |
```

</details>
