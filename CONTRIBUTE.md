# Contribute

Thank you for your interest in contributing to µcad!

Starting as a team of two we had to chose a way to bring code and documentation into sync.
Especially when writing a programming language!

So our so-called [MD-Tests](tests/markdown_test.rs) are [generated automatically](tests/microcad_markdown_test/lib.rs) out of the code from
within the documentation to check if it is correct.

Having this tool, we can make tested code examples which produce:

- Proper test result [with banners](#test-results-and-marks) near the documentation
- SVG/STL Output (2D) which my be shown within documentation
- Clear log for every test (e.g. [first_example.log](.test/first_example.log))
- A list of all [tests](doc/test_list.md)

Those test can be run with `cargo test`.
The produced output will be saved in folders called `.test` which is beside the source file of the test.

Outdated or removed tests will be cleaned up automatically but when in doubt use `cargo clean` and maybe some `find -name .test | xargs rm -r`.

We also commit the results to the repository to monitor any changes in our IDEs.

## Contribute Documentation

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

The user manual is the *point of truth* about what µcad is capable to do and what not.
This *document driven* approach guarantees to test each proper marked (see below) code example and show the test result in a banner above the test.

##### Documentation driven tests

One may insert *µcad* code into the *markdown* files, which then will get tested automatically if you run `cargo test` and name them like:

````md
```µcad,my_test
````

The *markdown* will be searched for any *µcad* code and appropriate *rust* tests will be  [generated](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/microcad_markdown_test).

##### Test modes

beside the name you may add a test mode (see table below):

````md
```µcad,my_test#fail
````

The tests will create `.test` folders beside the *markdown* files.
The tests will then copy an [image file (`*.svg`)](https://github.com/Rustfahrtagentur/microcad/tree/master/tests/images) for every test which signals the test result into the `.test` folder.
They can be included in the *markdown*, if you use this code:

````md
![test](.test/my_test.svg)
```µcad,my_test
````

##### Accessing test logs

You may also give the reader access to the logs by clicking on the banner with:

````md
[![test](.test/my_test.svg)](.test/my_test.log)
```µcad,my_test
````

##### Automatically update test banners

There is a [script](https://github.com/Rustfahrtagentur/microcad/tree/master/update_md_banner.sh) which updates all banners automatically.

##### Test results and marks

| Image                                            | MD Code Type | Mark         | Code                                     | What do do?            |
| ------------------------------------------------ | ------------ | ------------ | ---------------------------------------- | ---------------------- |
| ![fail_ok](tests/images/fail_ok.svg)             | `µcad`       | `#fail`      | fails intentionally                      | ok                     |
| ![fail_wrong](tests/images/fail_wrong.svg)       | `µcad`       | `#fail`      | fails but with wrong errors              | fix test or code       |
| ![fail](tests/images/fail.svg)                   | `µcad`       |              | fails                                    | fix test or code       |
| ![not_todo_fail](tests/images/not_todo_fail.svg) | `µcad`       | `#todo_fail` | Fails as expected but still marked to do | remove `#todo_`        |
| ![not_todo](tests/images/not_todo.svg)           | `µcad`       | `#todo`      | Succeeds but still marked to do          | remove `#todo`         |
| ![ok_fail](tests/images/ok_fail.svg)             | `µcad`       | `#fail`      | succeeds but should fail                 | find out why           |
| ![ok](tests/images/ok.svg)                       | `µcad`       |              | succeeds                                 | ok                     |
| ![parse_fail](tests/images/parse_fail.svg)       | `µcad`       | -            | Parsing has failed                       | fix grammar            |
| ![todo_fail](tests/images/todo_fail.svg)         | `µcad`       | `#todo_fail` | needs more work to fail                  | create issue/implement |
| ![todo](tests/images/todo.svg)                   | `µcad`       | `#todo`      | needs more work to succeed               | create issue/implement |
| -                                                | `µcad`       | `#no-test`   | Ignore completely                        | yolo!                  |
| -                                                | -            | -            | Ignore completely                        | yolo!                  |
| -                                                | *(other)*    | -            | Ignore completely                        | yolo!                  |

##### Mark errors and warnings

Code lines which intentionally produce errors must be marked with `// error` to make the test succeed.
Code lines which shall produce warnings can be marked with `// warning` to check if those warnings are happening.
Any unmarked warnings will be ignored.

In the following example a warning and an error are marked with comments:

````md
```µcad,missed_property#fail
sketch Wheel(radius: Length) { // warning (no output)
    init( width: Length ) { } // error: Workbench plan incomplete. Missing properties: radius
}
Wheel(width = 1.0mm);
```
````
