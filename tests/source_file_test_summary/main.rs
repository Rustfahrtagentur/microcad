use std::io::Write;

# Test Summary

| Test file | SVG | Tree dump | Result |
|-----------|-----|-----------|--------|
| [algorithm/difference.µcad](test_cases/algorithm/difference.µcad) | <img src="output/algorithm/difference.µcad.svg" alt="difference.µcad" width="100"/> | [Tree](output/algorithm/difference.tree.dump)  | [:white_check_mark:](output/algorithm/difference.log) |

fn generate_table_header(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "| Test file | SVG | Tree dump | Result |\n")?;
    writeln!(w, "|-----------|-----|-----------|--------|")?;
    Ok(())
}

fn generate_table_row(w: &mut impl Write, test_case: &str) -> std::io::Result<()> {

    let log_file = format!("output/{test_case}.log");
    let svg_file = format!("output/{test_case}.svg");
    let tree_file = format!("output/{test_case}.tree.dump");

    writeln!(w, "| [{test_case}](test_cases/{test_case}.µcad) | <img src=\"output/{test_case}.svg\" alt=\"{test_case}\" width=\"100\"/> | [Tree](output/{test_case}.tree.dump)  | [:white_check_mark:](output/{test_case}.log) |")?;
    Ok(())
}


fn generate_mark_down_table() {
    let mut table = String::new();
    table.push_str("| Path | Line | Column | Description |\n");
    table.push_str("| ---- | ---- | ------ | ----------- |\n");
    table.push_str("| tests/source_file_test_summary/main.rs | 0 | 0 | Compare this snippet from lang/parse/module/module_definition.rs: |\n");
    table.push_str("| target/debug/build/microcad-tests-3ef491166e875fea/out/microcad_source_file_test.rs | 0 | 0 |  |\n");
    table.push_str("| tests/source_file_test/lib.rs | 0 | 0 |  |\n");
    table.push_str("| lang/parse/module/module_definition_body.rs | 0 | 0 |  |");
    println!("{}", table);
}


fn main() {}
