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
  - [Installation](#installation)
  - [Basic Example](#basic-example)
    - [Source Code Explanation](#source-code-explanation)
- [Documentation](#documentation)
- [Contribute](#contribute)
  - [Get Source Code](#get-source-code)
  - [Get External Libraries](#get-external-libraries)
  - [Build µcad](#build-µcad)
  - [Install µcad locally from source](#install-µcad-locally-from-source)
  - [Contributing to Documentation](#contributing-to-documentation)
    - [User Manual](#user-manual)
- [Test List](#test-list)
- [💚 Funded by](#-funded-by)

## Quick Start

*µcad* is programmed in [Rust](https://www.rust-lang.org/) which easily can be [installed](https://www.rust-lang.org/tools/install) on several operating systems.
You can try it out with an example by using the command line tool `microcad-cli`
which can be installed from [crates.io](https://crates.io) by using `cargo`.

**Note**: Currently µcad has no binary install packages so the only ways to install it are with [`cargo install`](#installation) or from the source code (see section [Contribute](#contribute)).

### Installation

First, install [Ninja Build](https://github.com/ninja-build/ninja) which is needed to compile the [manifold geometry library](https://github.com/elalish/manifold).
For example, *Debian* based *Linux* distributions use the following line:

```sh
sudo apt install ninja-build
```

To install the latest release of *µcad* via *cargo*, type:

```sh
cargo install microcad-cli
```

### Basic Example

After installing, you can run a basic example by typing:

```sh
microcad eval ./examples/lid.µcad
```

This will *evaluate* the input file and will calculate and output the volume of the geometry:

```console
Volume: 48.415571412489506cm³
```

The *evaluate* command will not export the output geometry. Instead, it will simply run the program,
which prints out the volume.

To generate an STL model file use the `export` command with an additional output file name:

```sh
microcad export ./examples/lid.µcad
```

The output file `lid.stl` can be displayed e.g. with [MeshLab](https://www.meshlab.net/).
The resulting STL model looks like this: ![Lid](examples/lid.png)

#### Source Code Explanation

The source file defines a *part* called `lid`, which instantiates two cylinders with different diameters and geometrically subtracts them with each other to generate a round [lid](https://rust.services/blog/20242511-mcad-lid/).

[![test](.test/first_example.png)](.test/first_example.log)

```µcad,first_example
// We have part called `lid` with three parameters
part lid(
    thickness = 1.6mm,
    inner_diameter = 16cm,
    height = 20mm,
) {
    // Calculate the outer diameter
    outer_diameter = 2 * thickness + inner_diameter;

    // Create two cylinders, one for the outer and one for the inner
    outer = std::geo3d::cylinder(d = outer_diameter, h = height);
    inner = std::ops::translate(z = thickness) std::geo3d::cylinder(d = inner_diameter, h = height);

    // Calculate the difference between two translated cylinders and output them
    outer - inner;
}

// `l` is the instance of the lid model
lid();
```

The STL file can now also be loaded into a slicer program like [Cura](https://ultimaker.com/software/ultimaker-cura) and print it on a 3D printer.

![Cura](doc/images/cura.png)

## Documentation

- [Description of language features](doc/README.md)
- [Language reference](doc/REFERENCE.md)
- [Basic concepts](doc/CONCEPTS.md)
- [Examples](doc/EXAMPLES.md)
- Code documentation:
  - [`microcad-lang` module](https://docs.rs/microcad-lang)
  - [`microcad-core` module](https://docs.rs/microcad-core)
  - [`microcad-export` module](https://docs.rs/microcad-export)
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

 One may insert *µcad* code into the *markdown* files, which then will get tested automatically if you run `cargo test` and name them like:

````md
```µcad,my_test
````

The *markdown* will be searched for any *µcad* code and appropriate *rust* tests will be  [generated](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/microcad_markdown_test).

beside the name you may add a test mode (see table below):

````md
```µcad,my_test#fail
````

The tests will create `.test` folders beside the *markdown* files.
The tests will then copy an [image file (`*.png`)](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/images) for every test which signals the test result into the `.test` folder.
They can be included in the *markdown*, if you use this code:

````md
![test](.test/my_test.png)
```µcad,my_test
````

| Image                                      | MD Code Type | Mark       | Code                            | What do do?            |
| ------------------------------------------ | ------------ | ---------- | ------------------------------- | ---------------------- |
| ![ok](tests/images/ok.png)                 | `µcad`       |            | succeeds                        | ok                     |
| ![fail](tests/images/fail.png)             | `µcad`       |            | fails                           | fix test or code       |
| ![ok_fail](tests/images/ok_fail.png)       | `µcad`       | `#fail`    | succeeds but should fail        | find out why           |
| ![fail_ok](tests/images/fail_ok.png)       | `µcad`       | `#fail`    | fails intentionally             | ok                     |
| ![todo](tests/images/todo.png)             | `µcad`       | `#todo`    | needs more work to succeed      | create issue/implement |
| ![not_todo](tests/images/not_todo.png)     | `µcad`       | `#todo`    | Succeeds but still marked to do | remove `#todo`         |
| ![parse_fail](tests/images/parse_fail.png) | `µcad`       | -          | Parsing has failed              | fix grammar            |
| -                                          | `µcad`       | `#no-test` | Ignore completely               | yolo!                  |
| -                                          | -            | -          | Ignore completely               | yolo!                  |
| -                                          | *(other)*    | -          | Ignore completely               | yolo!                  |

You may also give the reader access to the logs by clicking on the banner with:

````md
[![test](.test/my_test.png)](.test/my_test.log)
```µcad,my_test
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
