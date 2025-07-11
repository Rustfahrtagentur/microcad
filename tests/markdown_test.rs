// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub fn run_test(
    name: &str,
    mode: Option<&str>,
    banner: &std::path::Path,
    out: &std::path::Path,
    reference: &str,
) {
    let banner = &banner.to_string_lossy().escape_default().to_string();
    let out_name = &out.to_string_lossy().escape_default().to_string();
    let todo = mode == Some("todo") || mode == Some("todo_fail");

    use std::fs;
    use std::io;
    use std::io::Write;

    use microcad_builtin::*;
    use microcad_lang::diag::*;
    use microcad_lang::syntax::*;

    // remove generated files before updating
    let _ = fs::remove_file(banner);
    let _ = fs::remove_file(out_name);

    let _ = fs::hard_link("images/parse_fail.png", banner);

    // create log file
    let out = &mut fs::File::create(out_name).expect("cannot create log file");
    let out = &mut io::BufWriter::new(out);

    writeln!(out, "-- Test --\n  {name}\n  {reference}").expect("output error");

    // load and handle µcad source file
    let source_file_result = SourceFile::load_from_str(r##"{code}"##);

    match mode {
        // test is expected to fail?
        Some("fail") => match source_file_result {
            // test expected to fail failed at parsing?
            Err(err) => {
                writeln!(out, "-- Parse Error --").expect("output error");
                out.write_all(format!("{err}").as_bytes())
                    .expect("output error");
                writeln!(out).expect("output error");
                let _ = fs::remove_file(banner);
                let _ = fs::hard_link("images/fail_ok.png", banner);
                writeln!(out, "-- Test Result --\nFAILED AS EXPECTED").expect("output error");
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
                    out,
                    "-- Output --\n{}",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                writeln!(out, "-- Errors --").expect("internal error");
                context.write_diagnosis(out).expect("internal error");

                let _ = fs::remove_file(banner);

                // check if test expected to fail failed at evaluation
                match (eval, context.has_errors()) {
                    // evaluation had been aborted?
                    (Err(err), _) => {
                        let _ = fs::hard_link("images/fail_ok.png", banner);
                        writeln!(out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!("{err}");
                    }
                    // evaluation produced errors?
                    (_, true) => {
                        let _ = fs::hard_link("images/fail_ok.png", banner);
                        writeln!(out, "-- Test Result --\nFAILED AS EXPECTED")
                            .expect("output error");
                        log::debug!(
                            "there were {error_count} errors (see {out_name})",
                            error_count = context.error_count()
                        );
                    }
                    // test expected to fail but succeeds?
                    (_, _) => {
                        let _ = fs::hard_link("images/ok_fail.png", banner);
                        writeln!(out, "-- Test Result --\nOK BUT SHOULD FAIL")
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

                writeln!(out, "-- Parse Error --").expect("output error");
                out.write_all(format!("{err}").as_bytes())
                    .expect("No output error");
                writeln!(out).expect("output error");

                if todo {
                    let _ = fs::hard_link("images/todo.png", banner);
                    writeln!(out, "-- Test Result --\nFAIL (TODO)").expect("output error");
                } else {
                    let _ = fs::hard_link("images/fail.png", banner);
                    writeln!(out, "-- Test Result --\nFAIL").expect("output error");
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
                    out,
                    "-- Output --\n{}",
                    context.output().expect("capture error")
                )
                .expect("output error");

                // print any error
                writeln!(out, "-- Errors --").expect("internal error");
                context.write_diagnosis(out).expect("internal error");

                let _ = fs::remove_file(banner);

                // check if test awaited to succeed but failed at evaluation
                match (eval, context.has_errors(), todo) {
                    // test expected to succeed and succeeds with no errors
                    (Ok(node), false, false) => {
                        let _ = fs::hard_link("images/ok.png", banner);
                        writeln!(out, "-- Test Result --\nOK").expect("output error");
                        todo!("Export node")
                    }
                    // test is todo but succeeds with no errors
                    (Ok(_), false, true) => {
                        let _ = fs::hard_link("images/not_todo.png", banner);
                        writeln!(out, "-- Test Result --\nOK BUT IS TODO").expect("output error");
                    }
                    // Any error but todo
                    (_, _, true) => {
                        let _ = fs::hard_link("images/todo.png", banner);
                        writeln!(out, "-- Test Result --\nTODO").expect("output error");
                    }
                    // evaluation had been aborted?
                    (Err(err), _, _) => {
                        let _ = fs::hard_link("images/fail.png", banner);
                        out.write_all(format!("{err}").as_bytes())
                            .expect("No output error");
                        writeln!(out, "-- Test Result --\nFAIL").expect("output error");
                        panic!("ERROR: {err}")
                    }
                    // evaluation produced errors?
                    (_, true, _) => {
                        let _ = fs::hard_link("images/fail.png", banner);
                        writeln!(out, "-- Test Result --\nFAIL").expect("output error");
                        panic!(
                            "ERROR: there were {error_count} errors (see {out_name})",
                            error_count = context.error_count()
                        );
                    }
                }
            }
        },
    }
}
