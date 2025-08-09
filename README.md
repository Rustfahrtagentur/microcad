# µcad

[![Status](https://github.com/Rustfahrtagentur/microcad/actions/workflows/rust.yml/badge.svg)](https://github.com/Rustfahrtagentur/microcad/actions)
[![Crates.io](https://img.shields.io/crates/v/microcad-cli.svg)](https://crates.io/crates/microcad-cli)
[![Documentation](https://docs.rs/microcad-cli/badge.svg)](https://docs.rs/microcad-cli/)
[![Codecov](https://codecov.io/github/Rustfahrtagentur/mcad/coverage.svg?branch=main)](https://codecov.io/gh/Rustfahrtagentur/microcad)
[![Dependency status](https://deps.rs/repo/github/Rustfahrtagentur/mcad/status.svg)](https://deps.rs/repo/github/Rustfahrtagentur/microcad)

![µcad Logo](doc/images/logo.png)

µcad (pronounced *microcad*) is a description language for modeling parameterizable geometric objects.
Simple basic shapes can be composed to create complex geometries which then can be rendered into STL or SVG files for 3D printing or CNC milling.

**Note**: This project is in an early stage of development and is not yet feature complete. Feel free to [contribute](#contribute) by opening issues or pull requests.

## Content

- [Content](#content)
- [Quick Start](#quick-start)
- [Hello World example](#hello-world-example)
- [Installation](#installation)
- [Command line usage](#command-line-usage)
- [Documentation](#documentation)
- [Contribute](#contribute)
  - [Get Source Code](#get-source-code)
  - [Get External Libraries](#get-external-libraries)
  - [Build µcad](#build-µcad)
  - [Install µcad locally from source](#install-µcad-locally-from-source)
  - [Contributing to Documentation](#contributing-to-documentation)
    - [User Manual](#user-manual)
      - [Documentation driven tests](#documentation-driven-tests)
      - [Test modes](#test-modes)
      - [Accessing test logs](#accessing-test-logs)
      - [Automatically update test banners](#automatically-update-test-banners)
      - [Test results and marks](#test-results-and-marks)
      - [Mark errors and warnings](#mark-errors-and-warnings)
- [Test List](#test-list)
- [💚 Funded by](#-funded-by)

## Quick Start

*µcad* is programmed in [Rust](https://www.rust-lang.org/) which easily can be [installed](https://www.rust-lang.org/tools/install) on several operating systems.
You can try it out with an example by using the command line tool `microcad-cli`
which can be installed from [crates.io](https://crates.io) by using `cargo`.

**Note**: Currently µcad has no binary install packages so the only ways to install it are with [`cargo install`](#installation) or from the source code (see section [Contribute](#contribute)).

## Hello World example

The following µcad source code defines a *part* called `csg_cube`, which has a body of a cube with rounded corners and three cylinders as holes:

![csg_cube](examples/csg_cube.png)

[![test](.test/first_example.svg)](.test/first_example.log)

```µcad,first_example
use std::math::*;
use std::ops::*;
use std::geo3d::*;

part csg_cube(size: Length) {
    body = sphere(r = size / 1.5) & cube(size);
    holes = cylinder(h = size, d = size / 1.5).orient([X,Y,Z]);
    body - holes;
}

csg_cube(50mm);
```

## Installation

First, install [Ninja Build](https://github.com/ninja-build/ninja) which is needed to compile the [manifold geometry library](https://github.com/elalish/manifold).
For example, *Debian* based *Linux* distributions use the following line:

```sh
sudo apt install ninja-build
```

To install the latest release of *µcad* via *cargo*, type:

```sh
cargo install microcad-cli
```

## Command line usage

After installing, you can run a basic example by typing:

```sh
microcad eval ./examples/lego_brick.µcad
```

This will *evaluate* the input file and will output the model tree.
The *evaluate* command will not export the output geometry.

To generate an STL model file use the `export` command with an additional output file name:

```sh
microcad export ./examples/lego_brick.µcad
```

The file [`lego_brick.µcad`](examples/lego_brick.µcad) generate a file called`lego_brick.stl` which can be displayed e.g. with [MeshLab](https://www.meshlab.net/).

The resulting STL model looks like this:

![Parametric Lego Brick](examples/lego_brick.png)

## Documentation

- [Description of language features](doc/README.md)
- [Language reference](doc/REFERENCE.md)
- [Basic concepts](doc/CONCEPTS.md)
- Code documentation:
  - [`microcad-lang` module](https://docs.rs/microcad-lang)
  - [`microcad-core` module](https://docs.rs/microcad-core)
  - [`microcad-export` module](https://docs.rs/microcad-export)
- [Markdown Tests](doc/test_list.md)
- [File Tests](TEST_SUMMARY.md)
- [Glossary](doc/GLOSSARY.md)

## Contribute

We welcome contributions to *µcad*, whether it is a bug report, feature request, or a pull request.

First install [*Git*](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
and [*Rust*](https://www.rust-lang.org/tools/install).

### Get Source Code

```sh
git clone https://github.com/Rustfahrtagentur/microcad.git
cd microcad
```

### Get External Libraries

```sh
git submodule init
git submodule update
```

### Build µcad

```sh
cargo build
```

### Install µcad locally from source

```sh
cargo install --path tools/cli
```

### Contributing to Documentation

#### User Manual

The user manual consists of several *markdown* files stored in the `/doc` folder, starting with the inside [`README.md`](doc/README.md).

The user manual is the *point of truth* about what µcad is capable to do and what not.
This *document driven* approach guarantees to test each proper marked (see below) code example and show the test result in a banner above the test.

##### Documentation driven tests

One may insert *µcad* code into the *markdown* files, which then will get tested automatically if you run `cargo test` and name them like:

````md
```µcad,my_test
````

The *markdown* will be searched for any *µcad* code and appropriate *rust* tests will be  [generated](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/microcad_markdown_test).

##### Test modes

beside the name you may add a test mode (see table below):

````md
```µcad,my_test#fail
````

The tests will create `.test` folders beside the *markdown* files.
The tests will then copy an [image file (`*.svg`)](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/images) for every test which signals the test result into the `.test` folder.
They can be included in the *markdown*, if you use this code:

````md
![test](.test/my_test.svg)
```µcad,my_test
````

##### Accessing test logs

You may also give the reader access to the logs by clicking on the banner with:

````md
[![test](.test/my_test.svg)](.test/my_test.log)
```µcad,my_test
````

##### Automatically update test banners

There is a [script](https://github.com/Rustfahrtagentur/microcad/tree/master/update_md_banner.sh) which updates all banners automatically 

##### Test results and marks

| Image                                            | MD Code Type | Mark         | Code                                     | What do do?            |
| ------------------------------------------------ | ------------ | ------------ | ---------------------------------------- | ---------------------- |
| ![fail_ok](tests/images/fail_ok.svg)             | `µcad`       | `#fail`      | fails intentionally                      | ok                     |
| ![fail_wrong](tests/images/fail_wrong.svg)       | `µcad`       | `#fail`      | fails but with wrong errors              | fix test or code       |
| ![fail](tests/images/fail.svg)                   | `µcad`       |              | fails                                    | fix test or code       |
| ![not_todo_fail](tests/images/not_todo_fail.svg) | `µcad`       | `#todo_fail` | Fails as expected but still marked to do | remove `#todo_`        |
| ![not_todo](tests/images/not_todo.svg)           | `µcad`       | `#todo`      | Succeeds but still marked to do          | remove `#todo`         |
| ![ok_fail](tests/images/ok_fail.svg)             | `µcad`       | `#fail`      | succeeds but should fail                 | find out why           |
| ![ok](tests/images/ok.svg)                       | `µcad`       |              | succeeds                                 | ok                     |
| ![parse_fail](tests/images/parse_fail.svg)       | `µcad`       | -            | Parsing has failed                       | fix grammar            |
| ![todo_fail](tests/images/todo_fail.svg)         | `µcad`       | `#todo_fail` | needs more work to fail                  | create issue/implement |
| ![todo](tests/images/todo.svg)                   | `µcad`       | `#todo`      | needs more work to succeed               | create issue/implement |
| -                                                | `µcad`       | `#no-test`   | Ignore completely                        | yolo!                  |
| -                                                | -            | -            | Ignore completely                        | yolo!                  |
| -                                                | *(other)*    | -            | Ignore completely                        | yolo!                  |

##### Mark errors and warnings

Code lines which intentionally produce errors must be marked with `// error` to make the test succeed.
Code lines which shall produce warnings can be marked with `// warning` to check if those warnings are happening.
Any unmarked warnings will be ignored.

In the following example a warning and an error are marked with comments:

````md
```µcad,missed_property#fail
sketch wheel(radius: Length) { // warning (no output)
    init( width: Length ) { } // error: misses to set `radius` from building plan
}
wheel(width = 1.0mm);
```
````

## Test List

There is a [list of all tests](doc/test_list.md) included in this documentation.

## 💚 Funded by

Thanks to the [Prototype Fund](https://www.prototypefund.de/) and the [Federal Ministry of Research, Technology and Space](https://www.bmbf.de/EN/) for funding this project in 2025.

<a href="https://prototypefund.de/en/"><img src="https://upload.wikimedia.org/wikipedia/commons/b/ba/Prototype_Fund_Logo_2025.svg" alt="Logo of the Prototype Fund" style="height: 70px;"></a>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
<a href="https://okfn.de/en/"><img src="https://upload.wikimedia.org/wikipedia/commons/4/4d/Open_Knowledge_Foundation_Deutschland_Logo.svg" alt="Logo of the Open Knowledge Foundation Germany" style="height: 100px;"></a>
&nbsp;&nbsp;
<a href="https://www.bmbf.de/EN/"><img src="https://upload.wikimedia.org/wikipedia/commons/d/df/BMFTR_Logo.svg" alt="Logo of the German Federal Ministry of Research, Technology and Space" style="height: 110px;"></a>
