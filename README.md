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

- [µcad](#µcad)
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

## Quick Start

*µcad* is programmed in [Rust](https://www.rust-lang.org/) which easily can be [installed](https://www.rust-lang.org/tools/install) on several operating systems.
You can try it out with an example by using the command line tool `microcad-cli`
which can be installed from [crates.io](https://crates.io) by using `cargo`.

**Note**: Currently µcad has no binary install packages so the only ways to install it are with [`cargo install`](#installation) or from the source code (see section [Contribute](#contribute)).

### Installation

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

The source file defines a *module* called `lid`, which instantiates two cylinders with different diameters and geometrically subtracts them with each other to generate a round [lid](https://rust.services/blog/20242511-mcad-lid/).

![test](.banner/first_example.png)

```µcad,first_example
// We have module called `lid` with three parameters
module lid(
    thickness = 1.6mm,
    inner_diameter = 16.0cm,
    height = 20.0mm,
) {
    // Calculate the outer diameter
    outer_diameter = 2.0 * thickness + inner_diameter;

    // Create two cylinders, one for the outer and one for the inner
    outer = std::geo3d::cylinder(d = outer_diameter, h = height);
    inner = std::translate(z = thickness) std::geo3d::cylinder(d = inner_diameter, h = height);

    // Calculate the difference between two translated cylinders and output them
    outer - inner;
}

// `l` is the instance of the lid model
l = lid();

// Print out the volume of the model instance
std::print("Volume: {l.volume() / 1000}cm³");

// Insert `l` into resulting object tree
std::export("lid.stl") l;
```

The above program prints out the following text and exports the model into a STL file called `lid.stl`.

```console
Volume: 48.415571412489506cm³
```

The STL file can now be loaded in a slicer program like [Cura](https://ultimaker.com/software/ultimaker-cura) and print it on a 3D printer.

![Cura](doc/images/cura.png)

## Documentation

- [Description of language features](doc/README.md)
- [Basic concepts](doc/CONCEPTS.md)
- Code documentation:
  - [`microcad-lang` module](https://docs.rs/microcad-lang)
  - [`microcad-core` module](https://docs.rs/microcad-core)
  - [`microcad-export` module](https://docs.rs/microcad-export)

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
