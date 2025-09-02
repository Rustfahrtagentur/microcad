# Create new microcad file

Before we design any geometry, we use the `microcad` command line tool to create a new µcad project:

```sh
microcad create lego_brick
```

This will create a file `lego_brick.µcad`.

Let's open this file in VSCode:

```µcad,create
// µcad generated file

sketch YourSketch( /* your building plan */ ) {
    // your code
}

// create YourSketch
YourSketch();
```

If we export the file with the command:

```sh
microcad export lego_brick
```

Nothing will be exported because the sketch does not contain any output geometry.
Therefore, let's add some geometry!
