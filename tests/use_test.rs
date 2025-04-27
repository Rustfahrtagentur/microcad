use microcad_lang::{eval::*, syntax::*};

#[test]
fn use_statements() {
    let input: std::path::PathBuf = "../examples/use.µcad".into();
    let mut context = EvalContext::new(
        SourceFile::load(input).expect("file {input}").resolve(None),
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
        None,
    );
    context.eval().expect("successful evaluation");

    // test::foo::bar alias baz
    assert!(context.fetch_local(&"baz".into()).is_ok());
    // std::geo2d::circle
    assert!(context.fetch_local(&"circle".into()).is_ok());
    // std::print from std/module.µcad
    assert!(context.fetch_local(&"print".into()).is_ok());
    // nodes from std::geo3d::*
    assert!(context.fetch_local(&"cube".into()).is_ok());
    assert!(context.fetch_local(&"sphere".into()).is_ok());
}
