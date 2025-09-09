# Make a parameterization concept

In this step, we want to make the Lego brick part fully parametric and reusable.
Via using parameters, we want to:

* control the number of knobs in both directions
* control the brick's height
* create a reusable Lego brick library so that we can write this:

    ```µcad
    lego_brick::LegoBrick(rows = 2, columns = 2, base_height = 9.6mm * 2);
    ```

* All our sketches shall lay beside that `LegoBrick` inside a new module `lego_brick`.

First, though, we need to find a way to place elements more generically than before.
