# Diagnostics

## Contents

- [Diagnostics](#diagnostics)
  - [Contents](#contents)
  - [Helpful error messages](#helpful-error-messages)
    - [Module name without parameter list](#module-name-without-parameter-list)

## Helpful error messages

This section lists some examples which all are failing with a specific error message.

### Module name without parameter list

The following example is missing a parameter list beside the module name:

[![test](.test/diag_module_name_without_parameter_list.png)](.test/diag_module_name_without_parameter_list.log)

```µcad,diag_module_name_without_parameter_list#fail
module f {
}
```

Correct would be:

[![test](.test/diag_module_name_without_parameter_list_fix.png)](.test/diag_module_name_without_parameter_list_fix.log)

```µcad,diag_module_name_without_parameter_list_fix#ok
module f() {
}
```
