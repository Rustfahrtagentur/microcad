// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::RenderResolution;
use microcad_export::{stl::StlExporter, svg::SvgExporter};
use microcad_lang::tree_display::FormatTree;

fn lines_with(code: &str, marker: &str) -> std::collections::HashSet<usize> {
    code.lines()
        .enumerate()
        .filter_map(|line| {
            if line.1.contains(marker) {
                Some(line.0 + 1)
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn init() {
    let _ = env_logger::builder().try_init();
}

#[allow(dead_code)]
pub fn run_test(
    name: &str,
    mode: &str,
    code: &str,
    banner: &str,
    log_filename: &str,
    out_filename: &str,
    reference: &str,
) {
    let todo = mode == "todo" || mode == "todo_fail";

    use std::fs;
    use std::io;
    use std::io::Write;

    use microcad_builtin::*;
    use microcad_lang::diag::*;
    use microcad_lang::syntax::*;

    crate::markdown_test::init();

    log::info!("Running test '{name}':\n\n{code}");

    // remove generated files before updating
    let _ = fs::remove_file(banner);
    let _ = fs::remove_file(log_filename);

    let _ = fs::hard_link("images/parse_fail.svg", banner);

    // create log file
    let log_out = &mut fs::File::create(log_filename).expect("cannot create log file");
    let log_out = &mut io::BufWriter::new(log_out);

    writeln!(log_out, "-- Test --\n  {name}\n  {reference}\n").expect("output error");
    writeln!(
        log_out,
        "-- Code --\n\n{}",
        code.lines()
            .enumerate()
            .map(|(n, line)| format!("{n:2}: {line}", n = n + 1))
            .collect::<Vec<_>>()
            .join("\n")
    )
    .expect("output error");
    writeln!(log_out).expect("output error");

    // load and handle µcad source file
    let source_file_result = SourceFile::load_from_str(code);

    match mode {
        // test is expected to fail?
        "fail" | "todo_fail" => match source_file_result {
            // test expected to fail failed at parsing?
            Err(err) => {
                writeln!(log_out, "-- Parse Error --").expect("output error");
                log_out
                    .write_all(format!("{err}").as_bytes())
                    .expect("output error");
                writeln!(log_out).expect("output error");
                let _ = fs::remove_file(banner);
                let _ = fs::hard_link("images/fail_ok.svg", banner);
                writeln!(
                    log_out,
                    "-- Test Result --\nFAILED AS EXPECTED (PARSE: Cannot check error line)"
                )
                .expect("output error");
                log::debug!("{err}")
            }
            // test expected to fail succeeded at parsing?
            Ok(source) => {
                // evaluate the code including µcad std library
                let mut context = ContextBuilder::from_source_captured(
                    source.clone(),
                    &["../lib".into(), "../doc/assets".into()],
                )
                .expect("resolve error")
                .build();
                let eval = context.eval();

                // get print output
                write!(
                    log_out,
                    "-- Output --\n{}",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                writeln!(log_out, "-- Errors --").expect("internal error");
                context.write_diagnosis(log_out).expect("internal error");

                if context.has_errors()
                    && (lines_with(code, "// error") != context.error_lines()
                        || lines_with(code, "// warning")
                            .iter()
                            .any(|l| !context.warning_lines().contains(l)))
                {
                    if todo {
                        let _ = fs::hard_link("images/todo_fail.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAIL(TODO)").expect("output error");
                    } else {
                        let _ = fs::hard_link("images/fail_wrong.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED BUT WITH WRONG ERRORS")
                            .expect("output error");
                        panic!("ERROR: test is marked to fail but fails with wrong errors");
                    }
                }

                let _ = fs::remove_file(banner);

                // check if test expected to fail failed at evaluation
                match (eval, context.has_errors(), todo) {
                    // evaluation had been aborted?
                    (Err(err), _, false) => {
                        let _ = fs::hard_link("images/fail_ok.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!("{err}");
                    }
                    // evaluation produced errors?
                    (_, true, false) => {
                        let _ = fs::hard_link("images/fail_ok.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!(
                            "there were {error_count} errors (see {log_filename})",
                            error_count = context.error_count()
                        );
                    }
                    // test fails as expected but is todo
                    (Err(_), _, true) | (_, true, true) => {
                        let _ = fs::hard_link("images/not_todo_fail.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED BUT IS TODO")
                            .expect("output error");
                    }
                    // test expected to fail but succeeds and is todo to fail?
                    (_, _, true) => {
                        let _ = fs::hard_link("images/todo_fail.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAIL(TODO)").expect("output error");
                    }
                    // test expected to fail but succeeds?
                    (_, _, false) => {
                        let _ = fs::hard_link("images/ok_fail.svg", banner);
                        writeln!(log_out, "-- Test Result --\nOK BUT SHOULD FAIL")
                            .expect("output error");
                        panic!("ERROR: test is marked to fail but succeeded");
                    }
                }
            }
        },
        // test is expected to succeed?
        _ => match source_file_result {
            // test awaited to succeed and parsing failed?
            Err(err) => {
                let _ = fs::remove_file(banner);

                writeln!(log_out, "-- Parse Error --").expect("output error");
                log_out
                    .write_all(format!("{err}").as_bytes())
                    .expect("No output error");
                writeln!(log_out).expect("output error");

                if todo {
                    let _ = fs::hard_link("images/todo.svg", banner);
                    writeln!(log_out, "-- Test Result --\nFAIL (TODO)").expect("output error");
                } else {
                    let _ = fs::hard_link("images/fail.svg", banner);
                    writeln!(log_out, "-- Test Result --\nFAIL").expect("output error");
                    panic!("ERROR: {err}")
                }
            }
            // test awaited to succeed and parsing succeeds?
            Ok(source) => {
                // evaluate the code including µcad std library
                let mut context = ContextBuilder::from_source_captured(
                    source.clone(),
                    &["../lib".into(), "../doc/assets".into()],
                )
                .expect("resolve error")
                .build();
                let eval = context.eval();

                // get print output
                write!(
                    log_out,
                    "-- Output --\n{}",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                writeln!(log_out, "-- Errors --").expect("internal error");
                context.write_diagnosis(log_out).expect("internal error");

                let _ = fs::remove_file(banner);

                // check if test awaited to succeed but failed at evaluation
                match (eval, context.has_errors(), todo) {
                    // test expected to succeed and succeeds with no errors
                    (Ok(model), false, false) => {
                        use microcad_lang::model::{ExportCommand as Export, OutputType};

                        // get print output
                        write!(log_out, "-- Model --\n{}\n", FormatTree(&model))
                            .expect("output error");

                        let _ = fs::hard_link("images/ok.svg", banner);
                        writeln!(log_out, "-- Test Result --\nOK").expect("output error");
                        match model.final_output_type() {
                            OutputType::Geometry2D => {
                                Export {
                                    filename: format!("{out_filename}.svg").into(),
                                    resolution: RenderResolution::default(),
                                    exporter: Rc::new(SvgExporter),
                                }
                                .export(&model)
                                .expect("No error");
                            }
                            OutputType::Geometry3D => {
                                Export {
                                    filename: format!("{out_filename}.stl").into(),
                                    resolution: RenderResolution::coarse(),
                                    exporter: Rc::new(StlExporter),
                                }
                                .export(&model)
                                .expect("No error");
                            }
                            OutputType::NotDetermined => {}
                            _ => panic!("Invalid geometry output"),
                        }
                    }
                    // test is todo but succeeds with no errors
                    (Ok(_), false, true) => {
                        let _ = fs::hard_link("images/not_todo.svg", banner);
                        writeln!(log_out, "-- Test Result --\nOK BUT IS TODO")
                            .expect("output error");
                    }
                    // Any error but todo
                    (_, _, true) => {
                        let _ = fs::hard_link("images/todo.svg", banner);
                        writeln!(log_out, "-- Test Result --\nTODO").expect("output error");
                    }
                    // evaluation had been aborted?
                    (Err(err), _, _) => {
                        let _ = fs::hard_link("images/fail.svg", banner);
                        log_out
                            .write_all(format!("{err}").as_bytes())
                            .expect("No output error");
                        writeln!(log_out, "-- Test Result --\nFAIL").expect("output error");
                        panic!("ERROR: {err}")
                    }
                    // evaluation produced errors?
                    (_, true, _) => {
                        let _ = fs::hard_link("images/fail.svg", banner);
                        writeln!(log_out, "-- Test Result --\nFAIL").expect("output error");
                        panic!(
                            "ERROR: there were {error_count} errors (see {log_filename})",
                            error_count = context.error_count()
                        );
                    }
                }
            }
        },
    }
}
