use microcad_lang::eval::*;

#[test]
fn use_statements() {
    microcad_lang::env_logger_init();

    let mut context = EvalContext::from_source(
        "test_cases/context/locals.µcad",
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
    )
    .expect("context");
    context.eval().expect("successful evaluation");
}

#[test]
fn locals() {
    microcad_lang::env_logger_init();

    let mut context = EvalContext::from_source(
        "test_cases/context/locals.µcad",
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
    )
    .expect("context");
    context.eval().expect("successful evaluation");
}
