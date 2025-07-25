// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::{RenderResolution, Size2D};
use microcad_export::svg::SvgExporter;
use microcad_test_tools::Output;

#[allow(dead_code)]
pub fn run_test(name: &str, mode: &str, code: &str, output: &Output) {
    let todo = mode == "todo" || mode == "todo_fail";

    use std::fs;
    use std::io;
    use std::io::Write;

    use microcad_builtin::*;
    use microcad_lang::diag::*;
    use microcad_lang::syntax::*;

    // remove generated files before updating
    let _ = fs::remove_file(output.banner_path());
    let _ = fs::remove_file(output.log_path());

    let _ = fs::hard_link("images/parse_fail.png", output.banner_path());

    // create log file
    let log = &mut fs::File::create(output.log_path()).expect("cannot create log file");
    let log = &mut io::BufWriter::new(log);

    writeln!(
        log,
        "# Test [`{name}`]({reference})\n",
        reference = output.reference()
    )
    .expect("output error");

    // load and handle µcad source file
    let source_file_result = SourceFile::load_from_str(code);

    match mode {
        // test is expected to fail?
        "fail" => match source_file_result {
            // test expected to fail failed at parsing?
            Err(err) => {
                writeln!(log, "## Parse Error\n\n```,plain\n{err}```\n").expect("output error");
                let _ = fs::remove_file(output.banner_path());
                let _ = fs::hard_link("images/fail_ok.png", output.banner_path());
                writeln!(
                    log,
                    "## Test Result\n\n![FAILED AS EXPECTED]({banner_link})",
                    banner_link = output.banner_link()
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
                .build();
                let eval = context.eval();

                // get print output
                writeln!(
                    log,
                    "## Output\n\n```,plain\n{}```\n",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                writeln!(log, "## Errors\n\n```,plain\n{}```\n", context.diagnosis())
                    .expect("internal error");

                let _ = fs::remove_file(output.banner_path());

                // check if test expected to fail failed at evaluation
                match (eval, context.has_errors()) {
                    // evaluation had been aborted?
                    (Err(err), _) => {
                        let _ = fs::hard_link("images/fail_ok.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![FAILED AS EXPECTED]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                        log::debug!("{err}");
                    }
                    // evaluation produced errors?
                    (_, true) => {
                        let _ = fs::hard_link("images/fail_ok.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![FAILED AS EXPECTED]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                        log::debug!(
                            "there were {error_count} errors (see {log_path})",
                            error_count = context.error_count(),
                            log_path = output.log_path_str()
                        );
                    }
                    // test expected to fail but succeeds?
                    (_, _) => {
                        let _ = fs::hard_link("images/ok_fail.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![OK BUT SHOULD FAIL]({banner_link})",
                            banner_link = output.banner_link()
                        )
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
                let _ = fs::remove_file(output.banner_path());

                writeln!(log, "## Parse Error\n\n```,plain\n{err}```\n").expect("output error");
                log.write_all(format!("{err}").as_bytes())
                    .expect("No output error");
                writeln!(log).expect("output error");

                if todo {
                    let _ = fs::hard_link("images/todo.png", output.banner_path());
                    writeln!(
                        log,
                        "## Test Result\n\n![FAIL (TODO)]({banner_link})",
                        banner_link = output.banner_link()
                    )
                    .expect("output error");
                } else {
                    let _ = fs::hard_link("images/fail.png", output.banner_path());
                    writeln!(
                        log,
                        "## Test Result\n\n![FAIL]({banner_link})",
                        banner_link = output.banner_link()
                    )
                    .expect("output error");
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
                .build();
                let eval = context.eval();

                // get print output
                writeln!(
                    log,
                    "## Output\n\n```,plain\n{}```\n",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                // print any error
                writeln!(log, "## Errors\n\n```,plain\n{}```\n", context.diagnosis())
                    .expect("internal error");

                let _ = fs::remove_file(output.banner_path());

                // check if test awaited to succeed but failed at evaluation
                match (eval, context.has_errors(), todo) {
                    // test expected to succeed and succeeds with no errors
                    (Ok(model), false, false) => {
                        use microcad_lang::model::{ExportAttribute as Export, OutputType};

                        let _ = fs::hard_link("images/ok.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![OK]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                        match model.final_output_type() {
                            OutputType::Geometry2D => {
                                Export {
                                    filename: output.out_path(),
                                    resolution: RenderResolution::default(),
                                    exporter: Rc::new(SvgExporter),
                                    layers: vec![],
                                    size: Size2D::A4,
                                }
                                .export(&model)
                                .expect("No error");
                            }
                            OutputType::Geometry3D => todo!("Implement 3D export"),
                            OutputType::NotDetermined => {}
                            _ => panic!("Invalid geometry output"),
                        }
                    }
                    // test is todo but succeeds with no errors
                    (Ok(_), false, true) => {
                        let _ = fs::hard_link("images/not_todo.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![OK BUT IS TODO]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                    }
                    // Any error but todo
                    (_, _, true) => {
                        let _ = fs::hard_link("images/todo.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![TODO]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                    }
                    // evaluation had been aborted?
                    (Err(err), _, _) => {
                        let _ = fs::hard_link("images/fail.png", output.banner_path());
                        log.write_all(format!("{err}").as_bytes())
                            .expect("No output error");
                        writeln!(
                            log,
                            "## Test Result\n\n![FAIL]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                        panic!("ERROR: {err}")
                    }
                    // evaluation produced errors?
                    (_, true, _) => {
                        let _ = fs::hard_link("images/fail.png", output.banner_path());
                        writeln!(
                            log,
                            "## Test Result\n\n![FAIL]({banner_link})",
                            banner_link = output.banner_link()
                        )
                        .expect("output error");
                        panic!(
                            "ERROR: there were {error_count} errors (see {log_path})",
                            error_count = context.error_count(),
                            log_path = output.log_path_str()
                        );
                    }
                }
            }
        },
    }
}
