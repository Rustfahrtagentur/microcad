# Automated Test Generation from this Markdown Documentation

## Test Code in Documentation

All listings that appear within this *Markdown* documentation are tested automatically
when running the following line:

```sh
cargo test
```

## Test Generation

The output will be written into one file called (in which `*` stands for the *GUID* of your build):

* `target/debug/build/microcad-parser-*/out/md_test.rs`

### Generator Source Code

See [microcad_markdown_test](../parser/microcad_markdown_test/lib.rs) for the source code
of the generator.

### Generated Rust Test Code

#### Module structure

Within the generated code Rust's `mod' will be used to sort all tests into a module
structure which helps orienting which test is from which *Markdown* file.

The modules are generated in two steps:

* modules by relative file path of the *Markdown* file the code is in:
  
  `/doc/functions/fields.rs` leads to

    ```rust
    mod r#functions {
        mod r#fields {
            ..
        }
    }
    ```

* modules by breadcrumb name:
    `example.basic' leads to:

    ```rust
    mod r#example {
        mod r#basic {
            ..
        }
    }
    ```

* Of course one can use both together which would lead to:

    ```rust
    mod r#functions {
        mod r#fields {
            mod r#example {
                mod r#basic {
            ..
                }
            }
        }
    }
    ```

### Test Function
