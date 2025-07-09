# Assignments

Whenever you use a more complex *expression*, it is often worthwhile to store it in a variable so that it can
be used once or multiple times elsewhere.
In µcad, variables are always immutable which means that once they are set, their value cannot be reset in the same context.
Therefore, they differ from the variables known in other programming languages.

Every assignment in µcad is  a variable.
So this example defines the variable `a` which from then is a reserved name within the scope in which it was defined.

[![test](.test/assignment.png)](.test/assignment.log)

```µcad,assignment
a = 5;
b = a * 2;
std::debug::assert_eq(a,5);
std::debug::assert_eq(b,10);
```

Another assignment of a variable with the same name is not allowed.

[![test](.test/assignment_immutable.png)](.test/assignment_immutable.log)

```µcad,assignment_immutable#todo_fail
a = 5;
a = a * 2; // error: a already defined in this scope
```
