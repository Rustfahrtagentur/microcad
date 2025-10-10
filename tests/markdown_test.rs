// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang::{
    eval::{Capture, EvalContext},
    model::Model,
    syntax::SourceFile,
};

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

#[allow(dead_code, clippy::too_many_arguments)]
pub fn run_test(
    name: &str,
    params: &str,
    mode: &str,
    code: &str,
    banner: &str,
    log_filename: &str,
    out_filename: &str,
    reference: &str,
) {
    let todo = mode == "todo" || mode == "todo_fail";
    let hires = params == "hires";

    use std::fs;
    use std::io;
    use std::io::Write;

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
    let source_file_result = SourceFile::load_from_str(name, code);

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
                let mut context = create_context(&source);
                let eval = context.eval();

                report_output(log_out, &context);
                report_errors(log_out, &context);
                report_test_fail(log_out, &context, code, banner, todo);

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
                let mut context = create_context(&source);
                let eval = context.eval();

                report_output(log_out, &context);
                report_errors(log_out, &context);

                let _ = fs::remove_file(banner);

                // check if test awaited to succeed but failed at evaluation
                match (eval, context.has_errors(), todo) {
                    // test expected to succeed and succeeds with no errors
                    (Ok(model), false, false) => {
                        report_model(log_out, model, banner, out_filename, hires);
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

// evaluate the code including µcad std library
fn create_context(source: &Rc<SourceFile>) -> EvalContext {
    EvalContext::from_source(
        source.clone(),
        Some(microcad_builtin::builtin_module()),
        &["../lib", "../doc/assets"],
        Capture::new(),
        microcad_builtin::builtin_exporters(),
        microcad_builtin::builtin_importers(),
    )
    .expect("resolve error")
}

fn report_output(log_out: &mut std::io::BufWriter<&mut std::fs::File>, context: &EvalContext) {
    use std::io::Write;

    write!(
        log_out,
        "-- Output --\n{}",
        context.output().expect("capture error")
    )
    .expect("output error");
}

// print any error
fn report_errors(log_out: &mut std::io::BufWriter<&mut std::fs::File>, context: &EvalContext) {
    use microcad_lang::diag::Diag;
    use std::io::Write;

    writeln!(log_out, "-- Errors --").expect("internal error");
    context.write_diagnosis(log_out).expect("internal error");
}

fn report_test_fail(
    log_out: &mut std::io::BufWriter<&mut std::fs::File>,
    context: &EvalContext,
    code: &str,
    banner: &str,
    todo: bool,
) {
    use microcad_lang::diag::Diag;
    use std::io::Write;

    if context.has_errors()
        && (lines_with(code, "// error") != context.error_lines()
            || lines_with(code, "// warning")
                .iter()
                .any(|l| !context.warning_lines().contains(l)))
    {
        if todo {
            let _ = std::fs::hard_link("images/todo_fail.svg", banner);
            writeln!(log_out, "-- Test Result --\nFAIL(TODO)").expect("output error");
        } else {
            let _ = std::fs::hard_link("images/fail_wrong.svg", banner);
            writeln!(log_out, "-- Test Result --\nFAILED BUT WITH WRONG ERRORS")
                .expect("output error");
            panic!("ERROR: test is marked to fail but fails with wrong errors");
        }
    }
}

fn report_model(
    log_out: &mut std::io::BufWriter<&mut std::fs::File>,
    model: Model,
    banner: &str,
    out_filename: &str,
    hires: bool,
) {
    use microcad_core::RenderResolution;
    use microcad_export::{stl::StlExporter, svg::SvgExporter};
    use microcad_lang::{
        model::{ExportCommand as Export, OutputType},
        tree_display::FormatTree,
    };
    use std::io::Write;

    // get print output
    write!(log_out, "-- Model --\n{}\n", FormatTree(&model)).expect("output error");

    let _ = std::fs::hard_link("images/ok.svg", banner);
    writeln!(log_out, "-- Test Result --\nOK").expect("no output error");

    let export = match model.deduce_output_type() {
        OutputType::Geometry2D => Some(Export {
            filename: format!("{out_filename}.svg").into(),
            resolution: RenderResolution::default(),
            exporter: Rc::new(SvgExporter),
        }),
        OutputType::Geometry3D => Some(Export {
            filename: format!("{out_filename}.stl").into(),
            resolution: if hires {
                RenderResolution::default()
            } else {
                RenderResolution::coarse()
            },
            exporter: Rc::new(StlExporter),
        }),
        OutputType::NotDetermined => {
            writeln!(log_out, "Could not determine output type.").expect("output error");
            None
        }
        _ => panic!("Invalid geometry output"),
    };
    match export {
        Some(export) => match export.export(&model) {
            Ok(_) => writeln!(log_out, "Export successful."),
            Err(error) => writeln!(log_out, "Export error: {error}"),
        },
        None => writeln!(log_out, "Nothing will be exported."),
    }
    .expect("output error")
}
