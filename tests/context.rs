use microcad_lang::{eval::*, syntax::*};

#[test]
fn use_statements() {
    microcad_lang::env_logger_init();

    let input: std::path::PathBuf = "test_cases/context/use.µcad".into();
    let mut context = EvalContext::new(
        SourceFile::load(input).expect("file {input}").resolve(None),
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
        Box::new(Stdout),
    );
    context.eval().expect("successful evaluation");
}

#[test]
fn locals() {
    microcad_lang::env_logger_init();

    let input: std::path::PathBuf = "test_cases/context/locals.µcad".into();
    let mut context = EvalContext::new(
        SourceFile::load(input).expect("file {input}").resolve(None),
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
        Box::new(Stdout),
    );
    context.eval().expect("successful evaluation");
}
