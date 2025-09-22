#!/bin/bash

# Use this script to publish a new µcad version to crates.io
# Note: Execute this script from the root dir of this repository.

ARGS=$@ # All arguments (e.g. `--dry-run`) will be appended to `cargo publish` 

export MICROCAD_STD_DIR=`pwd`/lib/std
echo "Publishing microcad..."
echo "µcad std lib is located in: ${MICROCAD_STD_DIR}"

PACKAGES=(
    "microcad-core"
    "microcad-lang"
    "microcad-export"
    "microcad-import"
    "microcad-builtin"
    "microcad"
)

# Publish all packages.
for package in "${PACKAGES[@]}"; do
    cargo publish -p $package $ARGS
done
