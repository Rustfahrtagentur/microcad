// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::{RenderResolution, Size2D};
use microcad_export::svg::SvgExporter;

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

    // remove generated files before updating
    let _ = fs::remove_file(banner);
    let _ = fs::remove_file(log_filename);

    let _ = fs::hard_link("images/parse_fail.png", banner);

    // create log file
    let log_out = &mut fs::File::create(log_filename).expect("cannot create log file");
    let log_out = &mut io::BufWriter::new(log_out);

    writeln!(log_out, "-- Test --\n  {name}\n  {reference}").expect("output error");

    // load and handle µcad source file
    let source_file_result = SourceFile::load_from_str(code);

    match mode {
        // test is expected to fail?
        "fail" => match source_file_result {
            // test expected to fail failed at parsing?
            Err(err) => {
                writeln!(log_out, "-- Parse Error --").expect("output error");
                log_out
                    .write_all(format!("{err}").as_bytes())
                    .expect("output error");
                writeln!(log_out).expect("output error");
                let _ = fs::remove_file(banner);
                let _ = fs::hard_link("images/fail_ok.png", banner);
                writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED").expect("output error");
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

                // check if test expected to fail failed at evaluation
                match (eval, context.has_errors()) {
                    // evaluation had been aborted?
                    (Err(err), _) => {
                        let _ = fs::hard_link("images/fail_ok.png", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!("{err}");
                    }
                    // evaluation produced errors?
                    (_, true) => {
                        let _ = fs::hard_link("images/fail_ok.png", banner);
                        writeln!(log_out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!(
                            "there were {error_count} errors (see {log_filename})",
                            error_count = context.error_count()
                        );
                    }
                    // test expected to fail but succeeds?
                    (_, _) => {
                        let _ = fs::hard_link("images/ok_fail.png", banner);
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
                    let _ = fs::hard_link("images/todo.png", banner);
                    writeln!(log_out, "-- Test Result --\nFAIL (TODO)").expect("output error");
                } else {
                    let _ = fs::hard_link("images/fail.png", banner);
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
                    (Ok(node), false, false) => {
                        use microcad_lang::model_tree::{
                            ExportAttribute as Export, ModelNodeOutputType,
                        };

                        let _ = fs::hard_link("images/ok.png", banner);
                        writeln!(log_out, "-- Test Result --\nOK").expect("output error");
                        match node.final_output_type() {
                            ModelNodeOutputType::Geometry2D => {
                                Export {
                                    filename: out_filename.to_string().into(),
                                    resolution: RenderResolution::default(),
                                    exporter: Rc::new(SvgExporter),
                                    layers: vec![],
                                    size: Size2D::A4,
                                }
                                .export(&node)
                                .expect("No error");
                            }
                            ModelNodeOutputType::Geometry3D => todo!("Implement 3D export"),
                            ModelNodeOutputType::NotDetermined => {}
                            _ => panic!("Invalid geometry output"),
                        }
                    }
                    // test is todo but succeeds with no errors
                    (Ok(_), false, true) => {
                        let _ = fs::hard_link("images/not_todo.png", banner);
                        writeln!(log_out, "-- Test Result --\nOK BUT IS TODO")
                            .expect("output error");
                    }
                    // Any error but todo
                    (_, _, true) => {
                        let _ = fs::hard_link("images/todo.png", banner);
                        writeln!(log_out, "-- Test Result --\nTODO").expect("output error");
                    }
                    // evaluation had been aborted?
                    (Err(err), _, _) => {
                        let _ = fs::hard_link("images/fail.png", banner);
                        log_out
                            .write_all(format!("{err}").as_bytes())
                            .expect("No output error");
                        writeln!(log_out, "-- Test Result --\nFAIL").expect("output error");
                        panic!("ERROR: {err}")
                    }
                    // evaluation produced errors?
                    (_, true, _) => {
                        let _ = fs::hard_link("images/fail.png", banner);
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
