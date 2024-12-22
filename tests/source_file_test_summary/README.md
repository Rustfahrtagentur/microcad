# Test summary generator

This executable parsed the test files in `test_cases/` the output of the test suite in `output/` and
generates a summary of the test results as a markdown file in the µcad root dir `TEST_SUMMARY.md`.

The executable is run as follows:

```bash
cargo run --release --bin source_file_test_summary
```

You can open the generated `TEST_SUMMARY.md` file to see the results of the tests.
