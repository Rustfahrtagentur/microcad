# Operations

There are several operations which all convert a stack of objects into a resulting object.

## Explicit form

They all have the following form:

> *operation* `(` *parameters* `) {` *code* `}`

* *operation*: name of the operation
* *parameters*: list of parameters to setup the operation
* *code*: generates objects which are processed by the operation

## Operator form

Some operations can be written in operator form:

> *left*  *operator* *right*

Currently the following operation are available:

* [union](union.md)
* [difference](difference.md)
* [intersection](intersection.md)
* [hull](hull.md)
