# µcad

[![Status](https://github.com/Rustfahrtagentur/mcad/actions/workflows/rust.yml/badge.svg)](https://github.com/Rustfahrtagentur/mcad/actions)
[![Crates.io](https://img.shields.io/crates/v/mcad.svg)](https://crates.io/crates/mcad)
[![Documentation](https://docs.rs/mcad/badge.svg)](https://docs.rs/mcad/)
[![Codecov](https://codecov.io/github/Rustfahrtagentur/mcad/coverage.svg?branch=main)](https://codecov.io/gh/Rustfahrtagentur/mcad)
[![Dependency status](https://deps.rs/repo/github/Rustfahrtagentur/mcad/status.svg)](https://deps.rs/repo/github/Rustfahrtagentur/mcad)

![µcad Logo](logo.png)

µcad (pronounced *microcad*) is a description language for modeling parameterizable geometric objects.
Simple basic shapes can be composed to create complex geometries which then can be rendered into STL or SVG files for 3D printing or CNC milling.

**Note**: This project is in an early stage of development and is not yet feature complete. Feel free to contribute by opening issues or pull requests.

## Quick start

You can try out µcad with an example.
Install µcad from [crates.io](https://crates.io) by using cargo:

### Installation

*Note*: The microcad-cli binary crate is not yet available on crates.io, but you can install it from the source code.

To install the latest version of µcad via cargo, type:

```sh
cargo install microcad-cli
```

You can also build µcad from source by cloning the repository, see the `Contribute` section below.

### Basic example

You can run a basic example by typing:

```sh
microcad eval ./examples/lid.µcad
```

This will *evaluate* the input file and will calculate the volume of the geometry:

```console
Volume: 48.415571412489506cm³
```

The *evaluate* command will not export the output geometry. Instead, it will simply run the program, which prints out the volume.

To generate an STL model file called, use the `export` command with an additional output file name:

```sh
microcad export ./examples/lid.µcad
```

The output file `lid.stl`, can be displayed e.g. with [MeshLab](https://www.meshlab.net/).
The resulting STL model looks like this: ![Lid](examples/lid.png)

## Source code explanation

The source file defines a *module* called `lid`, which instantiates two cylinders with different diameters and subtract them with each other to generate a round [lid](https://rust.services/blog/20242511-mcad-lid/).

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

The program above will print out the following text and export the model to STL.

```console
Volume: 48.415571412489506cm³
```

We can now load the STL into a slicer program like [Cura](https://ultimaker.com/software/ultimaker-cura) and print it on a 3D printer.

## Contribute

We welcome contributions to µcad, whether it is a bug report, feature request, or a pull request.

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
