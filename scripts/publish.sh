#!/bin/bash

# Use this script to publish a new µcad version to crates.io
# Note: Execute this script from the root dir of this repository.

# Check if we are on a git tag and if it matches the crate version.

ARGS=$@ # All arguments (e.g. `--dry-run`) will be appended to `cargo publish` 
GIT_TAG=$(git describe --tags --exact-match 2>/dev/null)

if [ -z "$GIT_TAG" ]; then
    echo "Warning: You are NOT a on git tag, this means you can only publish in dry run mode!"
    ARGS+=" --dry-run"
else
    echo "You are on git tag: ${GIT_TAG}"
    CRATE_VERSION=v`cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'`
    
    if [ "$GIT_TAG" != "$CRATE_VERSION" ]; then
        echo "Error: Git tag ($GIT_TAG) does not match Cargo.toml version ($CRATE_VERSION)"
        echo "Create a tag with version ${CRATE_VERSION} first before you publish."
        exit 1
    fi
fi


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
