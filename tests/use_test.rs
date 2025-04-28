use microcad_lang::{eval::*, syntax::*};

#[test]
fn use_statements() {
    let input: std::path::PathBuf = "test_cases/use/use_test.µcad".into();
    let mut context = EvalContext::new(
        SourceFile::load(input).expect("file {input}").resolve(None),
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
        Box::new(Stdout),
    );
    context.eval().expect("successful evaluation");

    // test::foo::bar alias baz
    assert!(context.lookup(&"baz".try_into().expect("name")).is_ok());
    // std::geo2d::circle
    assert!(context.lookup(&"circle".try_into().expect("name")).is_ok());
    // std::print from std/module.µcad
    assert!(context.lookup(&"print".try_into().expect("name")).is_ok());
    // nodes from std::geo3d::*
    assert!(context.lookup(&"cube".try_into().expect("name")).is_ok());
    assert!(context.lookup(&"sphere".try_into().expect("name")).is_ok());

    // global node from module
    assert!(context
        .lookup(&"use_test::my_module".try_into().expect("valid name"))
        .is_ok());
}
