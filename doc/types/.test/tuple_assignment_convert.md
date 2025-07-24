# Test [`tuple_assignment_convert`](../doc/types/tuples.md#L75)

## Parse Error

```,plain
Parser error:  --> 2:17
  |
2 | (width, height) = array;
  |                 ^---
  |
  = expected EOI, COMMENT, list_element_access, tuple_element_access, attribute_access, add, subtract, multiply, divide, union, intersection, power_xor, greater_than, less_than, greater_equal, less_equal, equal, near, not_equal, and, or, xor, if_binary_op, else_binary_op, or method_call```

Parser error:  --> 2:17
  |
2 | (width, height) = array;
  |                 ^---
  |
  = expected EOI, COMMENT, list_element_access, tuple_element_access, attribute_access, add, subtract, multiply, divide, union, intersection, power_xor, greater_than, less_than, greater_equal, less_equal, equal, near, not_equal, and, or, xor, if_binary_op, else_binary_op, or method_call
## Test Result

![FAIL (TODO)](../doc/types/.test/tuple_assignment_convert.png)
