use crate::{ContextBuilder, NamespaceBuilder};
use microcad_lang::eval::*;

#[test]
fn context_namespace() {
    use microcad_lang::src_ref::*;

    let mut context = Context::default();

    let module = NamespaceBuilder::new("math")
        .add_value("pi", Value::Scalar(Refer::none(std::f64::consts::PI)))
        .build();

    context.add_module(module);

    let symbols = context
        .get_symbols_by_qualified_name(&"math::pi".into())
        .unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].id().unwrap(), "pi");
}

#[test]
fn test_assert() {
    use microcad_lang::parse::source_file::SourceFile;

    use std::str::FromStr;
    let source_file = match SourceFile::from_str(
        r#"
            std::assert(std::math::abs(-1.0) == 1.0);
        "#,
    ) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = ContextBuilder::new(source_file).with_std().build();

    match context.eval() {
        Ok(_) => {
            println!("Our assertion was successful as expected");
        }
        Err(err) => panic!("{err}"),
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! args {
    ($($name:ident: $ty:ident = $value:expr),*) => {&{
        use microcad_lang::src_ref::*;
        let mut map = ArgumentMap::new(SrcRef(None));
        $(map.insert(stringify!($name).into(), microcad_lang::eval::Value::$ty(microcad_lang::src_ref::Refer::none($value)));)*
        map
    }};
    () => {};
}

#[test]
fn difference_svg() {
    use crate::{algorithm::*, geo2d::*};
    use microcad_render::{svg::SvgRenderer, tree, Renderer2D};

    let difference = difference().unwrap();
    let group = tree::group();
    group.append(Circle::node(args!(radius: Scalar = 4.0)).unwrap());
    group.append(Circle::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    let file = std::fs::File::create("../test_output/std/difference.svg").unwrap();
    let mut renderer = SvgRenderer::default();
    renderer.set_output(Box::new(file)).unwrap();
    renderer.render_node(difference).unwrap();
}

#[test]
fn difference_stl() {
    use crate::algorithm;
    use microcad_export::stl::StlExporter;
    use microcad_lang::eval::ArgumentMap;

    let difference = algorithm::difference().unwrap();
    let group = microcad_render::tree::group();
    group.append(
        crate::geo3d::Cube::node(
            args!(size_x: Scalar = 4.0, size_y: Scalar = 4.0, size_z: Scalar = 4.0),
        )
        .unwrap(),
    );
    group.append(crate::geo3d::Sphere::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    use microcad_export::Exporter;
    let mut exporter = StlExporter::from_settings(&microcad_core::ExportSettings::with_filename(
        "../test_output/std/difference.stl".to_string(),
    ))
    .unwrap();

    exporter.export(difference).unwrap();
}
