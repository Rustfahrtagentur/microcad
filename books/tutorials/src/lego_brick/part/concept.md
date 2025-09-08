# Making a concept

Now we want to make the Lego brick part fully parametric and reusable.
We want to:

- control the number of knobs in both directions
- control the height as well
- create a reusable Lego brick library so that we can write this:

    ```µcad
    lego_brick::LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2);
    ```

- All our sketches shall lay beside that `LegoBrick` inside a new module `lego_brick`.

But first we need to get control over placing elements in a more generic way than we did before...
