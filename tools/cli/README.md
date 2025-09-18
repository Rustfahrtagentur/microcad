# microcad

This crate provides the command line interpreter of [µcad](http://microcad.xyz).

## Installation

To install the latest version via *cargo*, type:

```sh
cargo install microcad
```

## Run

Start the microcad CLI by typing `microcad` into your console.

```plain
microcad
```

This will give you an overview of the available functionality:

```plain
µcad Command Line Interface

Usage: microcad [OPTIONS] <COMMAND>

Commands:
  parse    Parse a µcad file
  resolve  Parse and resolve a µcad file
  eval     Parse and evaluate a µcad file
  export   Parse and evaluate and export a µcad file
  create   Create a new source file with µcad extension
  watch    Watch a µcad file
  help     Print this message or the help of the given subcommand(s)

Options:
  -T, --time                        Display processing time
  -P, --search-path <SEARCH_PATHS>  Paths to search for files [default: ./lib]
  -C, --config <CONFIG>             Load config from file
  -h, --help                        Print help
  -V, --version                     Print version
```

## Install standard library

In most cases you might want to use the *microcad standard library* (`std`).

This must be installed once by running the following command:

```sh
microcad install std
```

You're now ready to use µcad!

## Resources

- Tutorials:

  - <http://microcad.xyz>  
- Language documentation:

  - <http://github.com/Rustfahrtagentur/microcad/blob/master/doc/README.md>
