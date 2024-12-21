# Algorithms

There are several algorithms which all convert a stack of objects into a resulting object.

## Module form

They all have the following form:

> *algorithm* `(` *parameters* `) {` *code* `}`

* *algorithm*: name of the algorithm
* *parameters*: list of parameters to setup the algorithm
* *code*: generates objects which are processed by the algorithm

## Operator form

Some algorithms can be written in operator form:

> *left*  *operator* *right*

Currently the following algorithms are available:

* [union](union.md)
* [difference](difference.md)
* [intersection](intersection.md)
* [hull](hull.md)
